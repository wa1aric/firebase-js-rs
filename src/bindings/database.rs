use crate::app::FirebaseApp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Database;

    #[wasm_bindgen(method, js_name = database)]
    pub fn database(_: &FirebaseApp) -> Database;

    #[wasm_bindgen(method, js_name = ref)]
    pub fn r#ref(this: &Database, path: String) -> Ref;

    pub type Ref;

    pub type Snapshot;

    #[wasm_bindgen(method, js_name = val)]
    pub fn val(this: &Snapshot) -> JsValue;

    #[wasm_bindgen(method, js_name = on)]
    pub fn on(this: &Ref, event: String, callback: &Closure<dyn FnMut(Snapshot)>);
}
