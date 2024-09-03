use std::future::Future;
use std::ops::AddAssign;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_ticker::{Ticker, TickerFactory};

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

async fn speed_test_impl<T: TickerFactory>(timeout: i32) -> Result<u32, JsValue> {
    let times = Rc::new(WasmRefCell::new(0));
    let times_ = times.clone();
    let ticker = T::new(move || {
        times_.borrow_mut().add_assign(1)
    })?;
    ticker.start()?;
    wait(timeout)?.await?;
    ticker.stop();
    let times = *times.borrow();
    Ok(times)
}

use wasm_bindgen_test::*;
use wasm_ticker::factory::*;

wasm_bindgen_test_configure!(run_in_node_experimental );
wasm_bindgen_test_configure!(run_in_browser );

#[wasm_bindgen_test]
async fn speed_test() {
    const TIMEOUT: i32 = 10000;
    let duration = std::time::Duration::from_millis(TIMEOUT as u64);

    match speed_test_impl::<MessageChannelTickerFactory>(TIMEOUT).await {
        Ok(times) => {
            let interval = duration / times;
            console_log!("MessageChannelTicker: {}/{:?}, interval: {:?}", times, duration, interval);
        }
        _ => ()
    }

    match speed_test_impl::<ImmediateTickerFactory>(TIMEOUT).await {
        Ok(times) => {
            let interval = duration / times;
            console_log!("ImmediateTicker: {}/{:?}, interval: {:?}", times, duration, interval);
        }
        _ => ()
    }

    match speed_test_impl::<TimeoutTickerFactory>(TIMEOUT).await {
        Ok(times) => {
            let interval = duration / times;
            console_log!("TimeoutTicker: {}/{:?}, interval: {:?}", times, duration, interval);
        }
        _ => ()
    }

    match speed_test_impl::<AnimationFrameTickerFactory>(TIMEOUT).await {
        Ok(times) => {
            let interval = duration / times;
            console_log!("AnimationFrameTicker: {}/{:?}, interval: {:?}", times, duration, interval);
        }
        _ => ()
    }

    match speed_test_impl::<AutoTickerFactory>(TIMEOUT).await {
        Ok(times) => {
            let interval = duration / times;
            console_log!("AutoTicker: {}/{:?}, interval: {:?}", times, duration, interval);
        }
        _ => ()
    }
}
