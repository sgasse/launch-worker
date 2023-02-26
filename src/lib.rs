use js_sys::{Array, Object};
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

pub fn worker_from_runner(runner_name: &str, pkg_name: &str) -> Result<Worker, JsValue> {
    let pkg_name = pkg_name.to_lowercase().replace("-", "_");
    let worker_str = format!(
        "
        self.onmessage = (event) => {{
          // The web-worker lives in a different prefixed namespace than the main
          // thread. To make scripts from the main thread available, it needs to send
          // its URL, which we receive here.
          importScripts(event.data.url + '/pkg/{pkg_name}.js')
          console.log('Imported script in worker_from_runner')

          const {{ {runner_name} }} = wasm_bindgen

          console.log('Launching another runner')

          async function run_in_worker() {{
            // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
            await wasm_bindgen(event.data.url + '/pkg/{pkg_name}_bg.wasm')

            const runner = {runner_name}.new()
            runner.init()

            self.onmessage = async (event) => {{
              console.log('JS: Receive event')
              runner.onmessage(event)
            }}
          }}

          run_in_worker()
        }}
        "
    );
    let worker = worker_from_str(&worker_str)?;
    let site_url = get_url()?;
    let obj = create_obj(vec![("url", JsValue::from_str(&site_url))]);
    worker.post_message(&obj)?;
    Ok(worker)
}

pub fn get_url() -> Result<String, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let location = document.location().unwrap();

    let protocol = location.protocol()?;
    let host = location.host()?;

    Ok(format!("{protocol}//{host}"))
}

fn str_array_to_js_value(array: &[&str]) -> JsValue {
    JsValue::from(
        array
            .iter()
            .map(|&x| JsValue::from(x))
            .collect::<js_sys::Array>(),
    )
}

fn create_obj(iterable: Vec<(&str, JsValue)>) -> Object {
    let props = iterable
        .into_iter()
        .map(|(name, value)| [JsValue::from_str(name), value].iter().collect::<Array>())
        .collect::<Array>();
    js_sys::Object::from_entries(&JsValue::from(props)).unwrap()
}
