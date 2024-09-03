use wasm_bindgen::__rt::Lazy;
use wasm_bindgen::JsValue;
use crate::{TickerFactory, NamedTicker};
use crate::ticker::*;
use super::{message_channel::MessageChannelTickerFactory, timers::*};

pub(crate) enum SelectedTicker {
    MessageChannel,
    Immediate,
    Timeout,
    AnimationFrame,
    None,
}

pub(crate) static SELECTED_TICKER: Lazy<SelectedTicker> = Lazy::new(||
    if MessageChannelTicker::check() {
        SelectedTicker::MessageChannel
    } else if ImmediateTicker::check() {
        SelectedTicker::Immediate
    } else if TimeoutTicker::check() {
        SelectedTicker::Timeout
    } else if AnimationFrameTicker::check() {
        SelectedTicker::AnimationFrame
    } else {
        SelectedTicker::None
    }
);

/// Automatically construct an available [Ticker](crate::Ticker)
/// wrapped by [AutoTicker].
///
/// Detecting order:
/// - [MessageChannelTicker]
/// - [ImmediateTicker]
/// - [TimeoutTicker]
/// - [AnimationFrameTicker]
#[derive(Clone, Copy)]
pub struct AutoTickerFactory;
impl TickerFactory for AutoTickerFactory {
    type Output = AutoTicker;

    fn new(task: impl FnMut() + 'static) -> Result<Self::Output, JsValue> {
        match *SELECTED_TICKER {
            SelectedTicker::None => Err(JsValue::from_str("No available implementation detected")),
            SelectedTicker::MessageChannel => Result::map(
                MessageChannelTickerFactory::new(task),
                AutoTicker::MessageChannel
            ),
            SelectedTicker::Immediate => Result::map(
                ImmediateTickerFactory::new(task),
                AutoTicker::Immediate
            ),
            SelectedTicker::Timeout => Result::map(
                TimeoutTickerFactory::new(task),
                AutoTicker::Timeout
            ),
            SelectedTicker::AnimationFrame => Result::map(
                AnimationFrameTickerFactory::new(task),
                AutoTicker::AnimationFrame
            ),
        }
    }
}
