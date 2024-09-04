#![cfg(target_family = "wasm")]

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_ticker::{NamedTicker, Ticker, TickerFactory};

#[wasm_bindgen]
extern "C" {
    type Promise;
    #[wasm_bindgen(static_method_of = Promise)]
    fn withResolvers() -> Resolvers;

    type Resolvers;

    #[wasm_bindgen(method, getter)]
    fn promise(this: &Resolvers) -> js_sys::Promise;
    #[wasm_bindgen(method, getter)]
    fn resolve(this: &Resolvers) -> js_sys::Function;

    #[wasm_bindgen(catch, js_name = "setTimeout")]
    fn set_timeout(handler: &js_sys::Function, timeout: i32) -> Result<JsValue, JsValue>;
}

async fn wait(timeout: i32) -> Result<JsValue, JsValue> {
    let resolvers = Promise::withResolvers();
    let (promise, resolve) = (resolvers.promise(), resolvers.resolve());

    let cb = Closure::once(move || {
        resolve.call0(&JsValue::null()).unwrap();
    });

    set_timeout(cb.as_ref().unchecked_ref(), timeout)?;
    JsFuture::from(promise).await
}

async fn sync_test_impl<F: TickerFactory<Output: NamedTicker>>(
    interval: i32,
    times: u32,
) -> Result<u64, JsValue> {
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
        wait(interval).await?;
        let n = n.get();
        let m = m.get();
        assert_eq!(
            n,
            m,
            "Assertion failed for n({}) == m({}), at {:?}",
            n,
            m,
            Duration::from_millis(interval as u64) * i
        );
    }

    ticker.stop();
    Ok(n.get())
}

use wasm_bindgen_test::*;
use wasm_ticker::factory::*;

wasm_bindgen_test_configure!(run_in_node_experimental);
wasm_bindgen_test_configure!(run_in_browser);

const INTERVAL: i32 = 10000;
const TIMES: u32 = 6;

#[wasm_bindgen_test]
async fn message_channel() -> Result<(), JsValue> {
    sync_test_impl::<MessageChannelTickerFactory>(INTERVAL, TIMES).await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn timeout() -> Result<(), JsValue> {
    sync_test_impl::<TimeoutTickerFactory>(INTERVAL, TIMES).await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn immediate() -> Result<(), JsValue> {
    sync_test_impl::<ImmediateTickerFactory>(INTERVAL, TIMES).await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn animation_frame() -> Result<(), JsValue> {
    sync_test_impl::<AnimationFrameTickerFactory>(INTERVAL, TIMES).await?;
    Ok(())
}
