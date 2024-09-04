use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use wasm_ticker::{Ticker, TickerFactory};

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

async fn drop_test_impl<F: TickerFactory>() -> Result<(), JsValue> {
    let b = Rc::new(WasmRefCell::new(true));
    let b_ = b.clone();
    let ticker = F::new(move || {
        *b_.borrow_mut() = false;
    })?;
    ticker.start()?;
    drop(ticker);
    wait(50).await?;
    assert!(*b.borrow());
    Ok(())
}
use wasm_ticker::factory::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental );

#[wasm_bindgen_test]
async fn message_channel() -> Result<(), JsValue> {
    drop_test_impl::<MessageChannelTickerFactory>().await
}

#[wasm_bindgen_test]
async fn timeout() -> Result<(), JsValue> {
    drop_test_impl::<TimeoutTickerFactory>().await
}
