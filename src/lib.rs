#![cfg(target_family = "wasm")]

mod bindings;

/// Types implement [Ticker]
pub mod ticker;
/// Factory types implement [TickerFactory]
pub mod factory;

use wasm_bindgen::JsValue;

/// Running state of [Ticker]
#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Started,
    Stopped,
    Error(JsValue),
}

pub trait TickerFactory {
    type Output: Ticker + Clone + PartialEq + Eq;
    fn new(task: impl FnMut() + 'static) -> Result<Self::Output, JsValue>;
}

/// A [Ticker] queues callback as a [Task] to JavaScript event loop.
///
/// Instead of Microtasks or just stacking in,
/// [Task] won't block current host context and UI rendering thread.
/// See also <https://developer.mozilla.org/docs/Web/API/HTML_DOM_API/Microtask_guide/in_depth>
///
/// [Task]: https://developer.mozilla.org/docs/Web/API/HTML_DOM_API/Microtask_guide
pub trait Ticker {
    /// Current state
    fn state(&self) -> State;

    /// Start queuing on next tick, this doesn't block current context.
    fn start(&self) -> Result<(), JsValue>;

    /// Call callback once and start queuing.
    fn start_immediate(&self) -> Result<(), JsValue>;

    /// Stop executing, implementations may not force cancel queued task,
    /// or just set state to [State::Stopped].
    fn stop(&self);

    /// Simply queue task once.
    fn spawn(task: impl FnOnce() + 'static) -> Result<(), JsValue> where Self: Sized;

    /// Queue task once and wrap return value by [Promise](js_sys::Promise).
    ///
    /// Requires [`Promise.withResolvers`][withResolvers] method.
    ///
    /// [withResolvers]: https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers
    fn spawn_promise(task: impl FnOnce() -> Result<JsValue, JsValue> + 'static)
                     -> Result<js_sys::Promise, JsValue> where Self: Sized
    {
        let resolvers = bindings::__wasm_ticker_binding_promise_resolvers()?;
        let promise = resolvers.__wasm_ticker_binding_promise();
        let resolve = resolvers.__wasm_ticker_binding_resolve();
        let reject = resolvers.__wasm_ticker_binding_reject();
        Self::spawn(move || {
            match task() {
                Ok(r) => resolve.call1(&JsValue::null(), &r),
                Err(e) => reject.call1(&JsValue::null(), &e),
            }.unwrap();
        })?;
        Ok(promise)
    }
}

/// [Ticker] with specialized JavaScript API.
pub trait NamedTicker: Ticker {
    /// Check if [Self] is available on current JavaScript Runtime.
    fn check() -> bool;
}

/// Tickers using JavaScript timer APIs.
pub trait TimerTicker: NamedTicker {
    /// Returned token type of JavaScript timer API like `setTimeout`.
    ///
    /// In browsers, this usually is [Number](js_sys::Number),
    /// but in Node it could be a [Object](js_sys::Object).
    type Token: AsRef<JsValue> + Clone;

    /// Get clone of latest [Self::Token] if exists.
    fn token(&self) -> Option<Self::Token>;
}
