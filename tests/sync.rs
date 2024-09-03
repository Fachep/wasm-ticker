#![cfg(target_family = "wasm")]

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_ticker::{NamedTicker, Ticker, TickerFactory};

#[wasm_bindgen]
extern {
    type Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    fn withResolvers() -> Resolvers;

    type Resolvers;

    #[wasm_bindgen(method, getter)]
    fn promise(this: &Resolvers) -> js_sys::Promise;
    #[wasm_bindgen(method, getter)]
    fn resolve(this: &Resolvers) -> js_sys::Function;
    #[wasm_bindgen(method, getter)]
    fn reject(this: &Resolvers) -> js_sys::Function;

    #[wasm_bindgen(catch, js_name = "setTimeout")]
    fn set_timeout(
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<JsValue, JsValue>;
}

struct Fut(JsFuture, Closure<dyn FnMut()>);
impl Future for Fut {
    type Output = <JsFuture as Future>::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::pin::pin!(Box::pin(&mut self.0)).poll(cx)
    }
}

fn wait(timeout: i32) -> Result<Fut, JsValue> {
    let resolvers = Promise::withResolvers();
    let (promise, resolve) = (resolvers.promise(), resolvers.resolve());

    let cb = Closure::once(move || {
        resolve.call0(&JsValue::null()).unwrap();
    });
    let fut = Fut(JsFuture::from(promise), cb);

    set_timeout(
        fut.1.as_ref().unchecked_ref(),
        timeout
    )?;

    Ok(fut)
}

async fn sync_test_impl<F: TickerFactory<Output: NamedTicker>>(interval: i32, times: u32) -> Result<u64, JsValue> {
    if !F::Output::check() {
        return Ok(0);
    }
    let n = Rc::new(Cell::new(0u64));
    let n_ = n.clone();
    let m = Rc::new(Cell::new(0u64));
    let m_ = m.clone();
    let ticker = F::new(move || {
        unsafe { *n_.as_ref().as_ptr() += 1 }
        unsafe { *m_.as_ref().as_ptr() += 1 }
    })?;

    ticker.start()?;
    assert_eq!(n.get(), m.get());

    for i in 0..times {
        wait(interval)?.await?;
        let n = n.get();
        let m = m.get();
        assert_eq!(
            n, m,
            "Assertion failed for n({}) == m({}), at {:?}",
            n, m,
            Duration::from_millis(interval as u64) * i
        );
    }

    ticker.stop();
    Ok(n.get())
}

use wasm_bindgen_test::*;
use wasm_ticker::factory::*;

wasm_bindgen_test_configure!(run_in_node_experimental );
wasm_bindgen_test_configure!(run_in_browser );

#[wasm_bindgen_test]
async fn sync_test() -> Result<(), JsValue> {
    const INTERVAL: i32 = 10000;
    const TIMES: u32 = 6;

    let impl_name = "MessageChannelTicker";
    console_log!("Start testing for {}", impl_name);
    match sync_test_impl::<MessageChannelTickerFactory>(INTERVAL, TIMES).await? {
        0 => console_log!("{} is not supported on this platform.", impl_name),
        n => console_log!("{} passed with n = {}.", impl_name, n),
    };

    let impl_name = "ImmediateTicker";
    console_log!("Start testing for {}", impl_name);
    match sync_test_impl::<ImmediateTickerFactory>(INTERVAL, TIMES).await? {
        0 => console_log!("{} is not supported on this platform.", impl_name),
        n => console_log!("{} passed with n = {}.", impl_name, n),
    };

    let impl_name = "TimeoutTicker";
    console_log!("Start testing for {}", impl_name);
    match sync_test_impl::<TimeoutTickerFactory>(INTERVAL, TIMES).await? {
        0 => console_log!("{} is not supported on this platform.", impl_name),
        n => console_log!("{} passed with n = {}.", impl_name, n),
    };

    let impl_name = "AnimationFrameTicker";
    console_log!("Start testing for {}", impl_name);
    match sync_test_impl::<AnimationFrameTickerFactory>(INTERVAL, TIMES).await? {
        0 => console_log!("{} is not supported on this platform.", impl_name),
        n => console_log!("{} passed with n = {}.", impl_name, n),
    };

    Ok(())
}
