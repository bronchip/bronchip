use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{ReadableStream, ReadableStreamDefaultController, ReadableStreamDefaultReader};

#[derive(Serialize, Deserialize)]
pub struct ReadData {
    value: Vec<u8>,
    done: bool,
}

#[wasm_bindgen]
pub struct StreamSource {
    stream: ReadableStream,
}

#[wasm_bindgen]
impl StreamSource {
    #[wasm_bindgen(getter, js_name = type)]
    pub fn type_(&self) -> JsValue {
        JsValue::undefined()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(r: ReadableStream) -> StreamSource {
        StreamSource { stream: r }
    }

    pub fn start(&self, controller: ReadableStreamDefaultController) -> Promise {
        let reader = ReadableStreamDefaultReader::from(JsValue::from(self.stream.get_reader()));
        future_to_promise(stream(reader, controller))
    }
}

async fn stream(
    r: ReadableStreamDefaultReader,
    c: ReadableStreamDefaultController,
) -> Result<JsValue, JsValue> {
    loop {
        let chunk = JsFuture::from(r.read()).await.unwrap();
        let rd: ReadData = serde_wasm_bindgen::from_value(chunk).unwrap();
        if rd.done {
            break;
        }
        let s = std::str::from_utf8(&rd.value).unwrap();
        c.enqueue_with_chunk(&JsValue::from_str(s)).unwrap();
    }
    Ok(JsValue::undefined())
}
