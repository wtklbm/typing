use napi::bindgen_prelude::*;
use napi_derive::napi;
use enigo::{Enigo, Settings, Keyboard};
use std::cell::RefCell;

// 使用 thread_local 缓存 Enigo 实例
thread_local! {
    static ENIGO: RefCell<Enigo> = RefCell::new(Enigo::new(&Settings::default()).unwrap());
}

/// 一个跨平台的工具函数，用于模拟键盘输入。
#[napi]
pub fn typing(input: Either<String, Vec<String>>) -> Result<()> {
    // 获取缓存的 Enigo 实例
    ENIGO.with(|enigo| {
        let mut enigo = enigo.borrow_mut();

        // 将输入统一转换为单个字符串
        let text = match input {
            Either::A(text) => text, // 单个字符串直接使用
            Either::B(texts) => texts.join(""), // 字符串数组拼接为单个字符串
        };

        // 直接调用 enigo.text 并处理错误
        enigo.text(&text).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to simulate typing: {}", e),
            )
        })
    })
}
