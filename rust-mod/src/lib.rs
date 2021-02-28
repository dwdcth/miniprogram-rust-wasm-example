use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn hello_world() -> String {
    "hello_world".into()
}

#[wasm_bindgen]
pub fn md5(data: JsValue) -> JsValue {
    let data: String = JsString::from(data).into();
    let digest = md5::compute(data.as_bytes());
    let hex: JsString = JsString::from(format!("{:x}", digest).as_str());
    JsValue::from(JsString::from(hex))
}