use wasm_bindgen_test::__rt::detect::Runtime;
use wasm_bindgen_test::*;
use wasm_ticker::ticker::{
    AnimationFrameTicker, ImmediateTicker, MessageChannelTicker, TimeoutTicker,
};
use wasm_ticker::NamedTicker;

//wasm_bindgen_test_configure!(run_in_browser );

#[wasm_bindgen_test]
fn check_test() {
    #[derive(PartialEq, Debug)]
    struct Res {
        message_channel: bool,
        timeout: bool,
        immediate: bool,
        animation_frame: bool,
    }
    let rt = __rt::detect::detect();
    let correct = match rt {
        Runtime::Browser => Res {
            message_channel: true,
            timeout: true,
            immediate: false,
            animation_frame: true,
        },
        Runtime::Node => Res {
            message_channel: true,
            timeout: true,
            immediate: true,
            animation_frame: false,
        },
        Runtime::Worker => Res {
            message_channel: true,
            timeout: true,
            immediate: true,
            animation_frame: false,
        },
    };
    let res = Res {
        message_channel: MessageChannelTicker::check(),
        timeout: TimeoutTicker::check(),
        immediate: ImmediateTicker::check(),
        animation_frame: AnimationFrameTicker::check(),
    };
    assert_eq!(res, correct);
}
