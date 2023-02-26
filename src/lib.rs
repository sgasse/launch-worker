use wasm_bindgen::JsValue;
use web_sys::{Blob, BlobPropertyBag, Url, Worker};

pub fn worker_from_str(worker_str: &str) -> Result<Worker, JsValue> {
    let blob_parts = str_array_to_js_value(&[worker_str]);
    let blob = Blob::new_with_str_sequence_and_options(
        &blob_parts,
        BlobPropertyBag::new().type_("application/javascript"),
    )?;
    let url_obj = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url_obj)
}

fn str_array_to_js_value(array: &[&str]) -> JsValue {
    JsValue::from(
        array
            .iter()
            .map(|&x| JsValue::from(x))
            .collect::<js_sys::Array>(),
    )
}
