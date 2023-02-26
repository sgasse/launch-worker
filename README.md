# Launch web worker

Highly WIP. Current usage:

```rust
fn startup() {
    let worker = worker_from_runner("MyRunner", "my_pkg_name").unwrap();
}

#[wasm_bindgen]
pub struct MyRunner;

#[wasm_bindgen]
impl MyRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&self) {
        console::log_1(&JsValue::from_str("MyRunner initialized"));
    }

    pub fn onmessage(&mut self, msg: MessageEvent) {
        console::log_1(&format!("MyRunner received: {msg:?}").into());
    }
}
```
