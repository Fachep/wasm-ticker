wasm-ticker
---
Non-block tick executor for WebAssembly Rust.

Ticker callbacks queue as [Tasks] to JavaScript event loop.
Instead of Microtasks or just stacking in,
[Tasks] won't block current host context and UI rendering thread.
See also https://developer.mozilla.org/docs/Web/API/HTML_DOM_API/Microtask_guide/in_depth

[Tasks]: https://developer.mozilla.org/docs/Web/API/HTML_DOM_API/Microtask_guide

|                Ticker                 |           API           | Platform |      Interval<br/>Browser / Node      |
|:-------------------------------------:|:-----------------------:|:--------:|:-------------------------------------:|
|        [MessageChannelTicker]         |   [Channel Messaging]   |    *     |             \>4µs / \<1µs             |
|    [ImmediateTicker][TimerTickers]    |     [setImmediate]      |   Node   |                 \~1µs                 |
|     [TimeoutTicker][TimerTickers]     |      [setTimeout]       |    *     | [\~4ms][setTimeout interval] / \~14ms |
| [AnimationFrameTicker][TimerTickers]  | [requestAnimationFrame] | Browser  |          According to device          |
|             [AutoTicker]              |      One of above       |    *     |                  N/A                  |

[MessageChannelTicker]: src/ticker/message_channel.rs
[TimerTickers]: src/ticker/timers.rs
[AutoTicker]: src/ticker/auto.rs

[Channel Messaging]: https://developer.mozilla.org/docs/Web/API/Channel_Messaging_API
[setTimeout]: https://developer.mozilla.org/docs/Web/API/setTimeout
[requestAnimationFrame]: https://developer.mozilla.org/docs/Web/API/Window/requestAnimationFrame
[setImmediate]: https://nodejs.org/en-us/learn/asynchronous-work/understanding-setimmediate

[setTimeout interval]: https://developer.mozilla.org/docs/Web/API/setTimeout#reasons_for_delays_longer_than_specified

### Speed Tests:
```shell
wasm-pack test --node --release --test speed
# MessageChannelTicker: 10726000/10s, interval: 932ns
# ImmediateTicker: 9051901/10s, interval: 1.104µs
# TimeoutTicker: 664/10s, interval: 15.06024ms
# AutoTicker: 9879000/10s, interval: 1.012µs

wasm-pack test --chrome --release --test speed # with devtools closed
# MessageChannelTicker: 2359423/10s, interval: 4.238µs
# TimeoutTicker: 2140/10s, interval: 4.672897ms
# AnimationFrameTicker: 1438/10s, interval: 6.954102ms
# AutoTicker: 2405336/10s, interval: 4.157µs

wasm-pack test --chrome --release --test speed # with devtools opened
# MessageChannelTicker: 847715/10s, interval: 11.796µs
# TimeoutTicker: 2134/10s, interval: 4.686035ms
# AnimationFrameTicker: 1441/10s, interval: 6.939625ms
# AutoTicker: 860572/10s, interval: 11.62µs
```
