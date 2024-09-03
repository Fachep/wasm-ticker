use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use web_sys::MessageChannel;
use crate::ticker::MessageChannelTicker;
use crate::{TickerFactory, State};

/// Factory type for [MessageChannelTicker].
#[derive(Clone, Copy)]
pub struct MessageChannelTickerFactory;

impl TickerFactory for MessageChannelTickerFactory {
    type Output = MessageChannelTicker;

    fn new(mut task: impl FnMut() + 'static) -> Result<Self::Output, JsValue> {
        let channel = MessageChannel::new()?;
        let state = Rc::new(WasmRefCell::new(State::Stopped));
        let port1 = channel.port1();
        let port2 = channel.port2();

        let state_ = state.clone();
        let port2_ = port2.clone();
        let cb = Closure::new(move || {
            if let State::Started = *state_.borrow() {
                task();
                if let Err(e) = port2_.post_message(&JsValue::null()) {
                    *state_.borrow_mut() = State::Error(e)
                }
            }
        });

        let state_ = state.clone();
        let cb_err = Closure::new(move |e: JsValue| {
            *state_.borrow_mut() = State::Error(e);
        });

        let ticker = MessageChannelTicker {
            port1: port1.clone(),
            port2,
            state,
            cb: Rc::new(cb),
            cb_err: Rc::new(cb_err),
        };
        port1.set_onmessage(Some(ticker.cb.as_ref().as_ref().unchecked_ref()));
        port1.set_onmessageerror(Some(ticker.cb_err.as_ref().as_ref().unchecked_ref()));
        Ok(ticker)
    }
}
