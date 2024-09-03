use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use web_sys::{MessageChannel, MessagePort};
use crate::{Ticker, NamedTicker, State};

/// Constructed by [MessageChannelTickerFactory](crate::factory::MessageChannelTickerFactory).
///
/// Requires [Channel Messaging API](https://developer.mozilla.org/docs/Web/API/Channel_Messaging_API)
#[derive(Clone)]
pub struct MessageChannelTicker {
    pub(crate) port1: MessagePort,
    pub(crate) port2: MessagePort,
    pub(crate) state: Rc<WasmRefCell<State>>,
    pub(crate) cb: Rc<Closure<dyn FnMut()>>,
    pub(crate) cb_err: Rc<Closure<dyn FnMut(JsValue)>>,
}

impl PartialEq for MessageChannelTicker {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
    }
}

impl Eq for MessageChannelTicker {}

impl Drop for MessageChannelTicker {
    fn drop(&mut self) {
        self.port1.set_onmessage(None);
        self.port1.close();
        self.port2.close();
    }
}

impl Ticker for MessageChannelTicker {
    #[inline]
    fn state(&self) -> State {
        self.state.borrow().clone()
    }

    fn start(&self) -> Result<(), JsValue> {
        match *self.state.borrow() {
            State::Started => return Err("Ticker started".into()),
            State::Stopped => (),
            State::Error(ref e) => return Err(e.clone()),
        };
        *self.state.borrow_mut() = State::Started;
        self.port2
            .post_message(&JsValue::null())
            .map_err(|e| {
                *self.state.borrow_mut() = State::Error(e.clone());
                e
            })
    }

    fn start_immediate(&self) -> Result<(), JsValue> {
        match *self.state.borrow() {
            State::Started => return Err("Ticker started".into()),
            State::Stopped => (),
            State::Error(ref e) => return Err(e.clone()),
        };
        *self.state.borrow_mut() = State::Started;
        self.port1.onmessage().as_ref().unwrap()
            .call0(&JsValue::null())
            .map_err(|e| {
                e
            })
            .map(|_| ())
    }

    fn stop(&self) {
        if self.state.borrow().eq(&State::Started) {
            *self.state.borrow_mut() = State::Stopped;
        }
    }

    fn spawn(task: impl FnOnce() + 'static) -> Result<(), JsValue> {
        let cb = Closure::once_into_js(task);
        let channel = MessageChannel::new()?;
        let port1 = channel.port1();
        let port2 = channel.port2();
        port1.set_onmessage(Some(cb.unchecked_ref()));
        port2.post_message(&JsValue::null())?;
        Ok(())
    }
}

impl NamedTicker for MessageChannelTicker {
    fn check() -> bool {
        static mut RET: Option<bool> = None;
        unsafe {
            *RET.get_or_insert_with(|| { MessageChannel::new().is_ok() })
        }
    }
}
