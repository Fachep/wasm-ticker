use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use crate::{Ticker, NamedTicker, TimerTicker, State};

macro_rules! timer_ticker_impl {
    {
        #[$($Attrs:tt)*]
        Ticker = $T:ident,
        Token = $TToken:ty,
        Check = $FCheck:path,
        Timer = $FTimer:path,
        Canceller = $FClear:path,
    } => {
        #[derive(Clone)]
        #[$($Attrs)*]
        pub struct $T {
            pub(crate) token: Rc<WasmRefCell<Option<$TToken>>>,
            pub(crate) state: Rc<WasmRefCell<State>>,
            pub(crate) cb: Rc<Closure<dyn FnMut()>>,
        }

        impl PartialEq for $T {
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.state, &other.state)
            }
        }

        impl Eq for $T {}

        impl Drop for $T {
            fn drop(&mut self) {
                if let Some(token) = self.token.borrow_mut().take() {
                    $FClear(token)
                }
            }
        }

        impl Ticker for $T {
            fn state(&self) -> State {
                self.state.borrow().clone()
            }

            fn start(&self) -> Result<(), JsValue> {
                let state = &mut *self.state.borrow_mut();
                match state {
                    State::Started => Err("Ticker started".into()),
                    State::Stopped => {
                        *state = State::Started;
                        self.token.borrow_mut()
                            .replace($FTimer(self.cb.as_ref().as_ref().unchecked_ref())?);
                        Ok(())
                    },
                    State::Error(e) => Err(e.clone()),
                }
            }

            fn start_immediate(&self) -> Result<(), JsValue> {
                let mut state = self.state.borrow_mut();
                match &*state {
                    State::Started => Err("Ticker started".into()),
                    State::Stopped => {
                        *state = State::Started;
                        drop(state);
                        self.cb.as_ref().as_ref()
                            .unchecked_ref::<js_sys::Function>()
                            .call0(&JsValue::null())?;
                        Ok(())
                    },
                    State::Error(e) => Err(e.clone()),
                }
            }

            fn stop(&self) {
                if self.state.borrow().eq(&State::Started) {
                    if let Some(token) = self.token.borrow_mut().take() {
                        $FClear(token)
                    }
                    *self.state.borrow_mut() = State::Stopped;
                }
            }

            fn spawn(task: impl FnOnce() + 'static) -> Result<(), JsValue> {
                let cb = Closure::once_into_js(task);
                $FTimer(cb.unchecked_ref())?;
                Ok(())
            }
        }

        impl NamedTicker for $T {
            fn check() -> bool {
                $FCheck()
            }
        }

        impl TimerTicker for $T {
            type Token = $TToken;

            fn token(&self) -> Option<Self::Token> {
                self.token.borrow().clone()
            }
        }
    };
}


use crate::bindings::{
    __wasm_ticker_binding_clear_timeout as clearTimeout,
    __wasm_ticker_binding_set_timeout as setTimeout,
    has_set_timeout,
    TimeoutToken
};
timer_ticker_impl!{
    #[doc = "Constructed by [TimeoutTickerFactory](crate::factory::TimeoutTickerFactory)."]
    Ticker = TimeoutTicker,
    Token = TimeoutToken,
    Check = has_set_timeout,
    Timer = setTimeout,
    Canceller = clearTimeout,
}

use crate::bindings::{
    __wasm_ticker_binding_clear_immediate as clearImmediate,
    __wasm_ticker_binding_set_immediate as setImmediate,
    has_set_immediate,
    ImmediateToken
};
timer_ticker_impl!{
    #[doc = "Constructed by [ImmediateTickerFactory](crate::factory::ImmediateTickerFactory).\
    Available in NodeJs."]
    Ticker = ImmediateTicker,
    Token = ImmediateToken,
    Check = has_set_immediate,
    Timer = setImmediate,
    Canceller = clearImmediate,
}

use crate::bindings::{
    __wasm_ticker_binding_cancel_animation_frame as cancelAnimationFrame,
    __wasm_ticker_binding_request_animation_frame as requestAnimationFrame,
    has_request_animation_frame,
    AnimationFrameToken
};
timer_ticker_impl!{
    #[doc = "Constructed by [AnimationFrameTickerFactory](crate::factory::AnimationFrameTickerFactory).\
    Available in browser Window context."]
    Ticker = AnimationFrameTicker,
    Token = AnimationFrameToken,
    Check = has_request_animation_frame,
    Timer = requestAnimationFrame,
    Canceller = cancelAnimationFrame,
}
