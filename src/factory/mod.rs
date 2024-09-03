mod auto;
mod message_channel;
mod timers;

pub use auto::AutoTickerFactory;
pub use message_channel::MessageChannelTickerFactory;

/// Factory type for [AnimationFrameTicker](crate::ticker::AnimationFrameTicker).
pub use timers::AnimationFrameTickerFactory;

/// Factory type for [TimeoutTicker](crate::ticker::TimeoutTicker).
pub use timers::TimeoutTickerFactory;

/// Factory type for [ImmediateTicker](crate::ticker::ImmediateTicker).
pub use timers::ImmediateTickerFactory;

pub(crate) use auto::{SELECTED_TICKER, SelectedTicker};
