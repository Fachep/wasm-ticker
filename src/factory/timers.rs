use crate::{State, TickerFactory};
use std::mem::MaybeUninit;
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
            let cb_uninit: Rc<Closure<dyn FnMut()>> = Rc::new(unsafe {
                #[allow(invalid_value)]
                MaybeUninit::uninit().assume_init()
            });

            let state_ = state.clone();
            let token_ = token.clone();
            let cb_uninit_ = cb_uninit.clone();
            let cb: Closure<dyn FnMut()> = Closure::new(move || {
                if state_.borrow().eq(&State::Started) {
                    task();
                    match $FTimer(cb_uninit_.as_ref().as_ref().unchecked_ref()) {
                        Ok(token) => {
                            token_.borrow_mut().replace(token);
                        },
                        Err(e) => *state_.borrow_mut() = State::Error(e),
                    };
                }
            });
            let _ = MaybeUninit::new(std::mem::replace(
                #[allow(invalid_reference_casting)]
                unsafe {&mut *(&*cb_uninit as *const Closure<dyn FnMut()>).cast_mut()},
                cb
            ));
            Ok(Self::Output {
                token,
                state,
                cb: cb_uninit,
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
