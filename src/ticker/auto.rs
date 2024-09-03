use wasm_bindgen::prelude::*;
use crate::{Ticker, State};
use super::{message_channel::MessageChannelTicker, timers::*};

/// Constructed by [AutoTickerFactory](crate::factory::AutoTickerFactory).
///
/// Wrapping one of [ImmediateTicker], [TimeoutTicker],
/// [AnimationFrameTicker] or [MessageChannelTicker].
#[derive(Clone, Eq, PartialEq)]
pub enum AutoTicker {
    MessageChannel(MessageChannelTicker),
    Timeout(TimeoutTicker),
    Immediate(ImmediateTicker),
    AnimationFrame(AnimationFrameTicker),
}

impl Ticker for AutoTicker {
    fn state(&self) -> State {
        match self {
            AutoTicker::MessageChannel(t) => t.state(),
            AutoTicker::Timeout(t) => t.state(),
            AutoTicker::Immediate(t) => t.state(),
            AutoTicker::AnimationFrame(t) => t.state(),
        }
    }

    fn start(&self) -> Result<(), JsValue> {
        match self {
            AutoTicker::MessageChannel(t) => t.start(),
            AutoTicker::Timeout(t) => t.start(),
            AutoTicker::Immediate(t) => t.start(),
            AutoTicker::AnimationFrame(t) => t.start(),
        }
    }

    fn start_immediate(&self) -> Result<(), JsValue> {
        match self {
            AutoTicker::MessageChannel(t) => t.start_immediate(),
            AutoTicker::Timeout(t) => t.start_immediate(),
            AutoTicker::Immediate(t) => t.start_immediate(),
            AutoTicker::AnimationFrame(t) => t.start_immediate(),
        }
    }

    fn stop(&self) {
        match self {
            AutoTicker::MessageChannel(t) => t.stop(),
            AutoTicker::Timeout(t) => t.stop(),
            AutoTicker::Immediate(t) => t.stop(),
            AutoTicker::AnimationFrame(t) => t.stop(),
        }
    }

    fn spawn(task: impl FnOnce() + 'static) -> Result<(), JsValue> {
        use crate::factory::{SelectedTicker, SELECTED_TICKER};
        match *SELECTED_TICKER {
            SelectedTicker::MessageChannel => MessageChannelTicker::spawn(task),
            SelectedTicker::Immediate => ImmediateTicker::spawn(task),
            SelectedTicker::Timeout => TimeoutTicker::spawn(task),
            SelectedTicker::AnimationFrame => AnimationFrameTicker::spawn(task),
            SelectedTicker::None => Err(JsValue::from_str("No available implementation detected")),
        }
    }
}

impl From<AutoTicker> for Box<dyn Ticker> {
    fn from(value: AutoTicker) -> Self {
        match value {
            AutoTicker::MessageChannel(t) => Box::new(t),
            AutoTicker::Timeout(t) => Box::new(t),
            AutoTicker::Immediate(t) => Box::new(t),
            AutoTicker::AnimationFrame(t) => Box::new(t),
        }
    }
}
