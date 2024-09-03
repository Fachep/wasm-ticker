use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type ImmediateToken;

    #[wasm_bindgen(catch, js_name = setImmediate)]
    pub fn __wasm_ticker_binding_set_immediate(
        cb: &js_sys::Function,
    ) -> Result<ImmediateToken, JsValue>;
    #[wasm_bindgen(js_name = clearImmediate)]
    pub fn __wasm_ticker_binding_clear_immediate(token: ImmediateToken);

    #[derive(Clone)]
    pub type TimeoutToken;

    #[wasm_bindgen(catch, js_name = setTimeout)]
    pub fn __wasm_ticker_binding_set_timeout(
        cb: &js_sys::Function,
    ) -> Result<TimeoutToken, JsValue>;
    #[wasm_bindgen(js_name = clearTimeout)]
    pub fn __wasm_ticker_binding_clear_timeout(token: TimeoutToken);

    #[derive(Clone)]
    pub type AnimationFrameToken;

    #[wasm_bindgen(catch, js_name = requestAnimationFrame)]
    pub fn __wasm_ticker_binding_request_animation_frame(
        cb: &js_sys::Function,
    ) -> Result<AnimationFrameToken, JsValue>;
    #[wasm_bindgen(js_name = cancelAnimationFrame)]
    pub fn __wasm_ticker_binding_cancel_animation_frame(token: AnimationFrameToken);

    pub type Resolvers;

    #[wasm_bindgen(catch, js_namespace = Promise, js_name = withResolvers)]
    pub fn __wasm_ticker_binding_promise_resolvers() -> Result<Resolvers, JsValue>;

    #[wasm_bindgen(method, getter, js_name = promise)]
    pub fn __wasm_ticker_binding_promise(this: &Resolvers) -> js_sys::Promise;
    #[wasm_bindgen(method, getter, js_name = resolve)]
    pub fn __wasm_ticker_binding_resolve(this: &Resolvers) -> js_sys::Function;
    #[wasm_bindgen(method, getter, js_name = reject)]
    pub fn __wasm_ticker_binding_reject(this: &Resolvers) -> js_sys::Function;
}

pub fn has_set_immediate() -> bool {
    js_sys::Reflect::has(&js_sys::global(), &JsValue::from_str("setImmediate")).unwrap_or(false)
}

pub fn has_set_timeout() -> bool {
    js_sys::Reflect::has(&js_sys::global(), &JsValue::from_str("setTimeout")).unwrap_or(false)
}

pub fn has_request_animation_frame() -> bool {
    js_sys::Reflect::has(
        &js_sys::global(),
        &JsValue::from_str("requestAnimationFrame"),
    )
    .unwrap_or(false)
}
