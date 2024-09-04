use crate::{State, TickerFactory};
use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;

macro_rules! timer_ticker_factory_impl {
    {
        Ticker = $T:ty,
        TickerFactory = $TFactory:ident,
        Timer = $FTimer:path,
    } => {
    #[derive(Clone, Copy)]
    pub struct $TFactory;

    impl TickerFactory for $TFactory {
        type Output = $T;

        fn new(mut task: impl FnMut() + 'static) -> Result<Self::Output, JsValue> {
            let state = Rc::new(WasmRefCell::new(State::Stopped));
            let token = Rc::new(WasmRefCell::new(None));

            let state_ = state.clone();
            let token_ = token.clone();
            let cb = Rc::new_cyclic(move |weak| {
                let weak = weak.clone();
                Closure::new(move || {
                    if state_.borrow().eq(&State::Started) {
                        task();
                        if let Some(cb) = weak.upgrade() {
                            match $FTimer(AsRef::<Closure<dyn FnMut()>>::as_ref(&cb).as_ref().unchecked_ref()) {
                                Ok(token) => {
                                    token_.borrow_mut().replace(token);
                                },
                                Err(e) => {
                                    *state_.borrow_mut() = State::Error(e);
                                }
                            }
                        }
                    }
                })
            });
            Ok(Self::Output {
                token,
                state,
                cb,
            })
        }
    }
    };
}

timer_ticker_factory_impl! {
    Ticker = crate::ticker::ImmediateTicker,
    TickerFactory = ImmediateTickerFactory,
    Timer = crate::bindings::__wasm_ticker_binding_set_immediate,
}

timer_ticker_factory_impl! {
    Ticker = crate::ticker::TimeoutTicker,
    TickerFactory = TimeoutTickerFactory,
    Timer = crate::bindings::__wasm_ticker_binding_set_timeout,
}

timer_ticker_factory_impl! {
    Ticker = crate::ticker::AnimationFrameTicker,
    TickerFactory = AnimationFrameTickerFactory,
    Timer = crate::bindings::__wasm_ticker_binding_request_animation_frame,
}
