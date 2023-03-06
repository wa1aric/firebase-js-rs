use crate::{app::FirebaseApp, event::Event};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Database;

    #[wasm_bindgen(method, js_name = database)]
    pub fn database(_: &FirebaseApp) -> Database;

    #[wasm_bindgen(method, js_name = ref)]
    pub fn r#ref(this: &Database, path: String) -> Ref;

    #[derive(Clone)]
    pub type Ref;

    pub type Snapshot;

    #[wasm_bindgen(method, js_name = val)]
    pub fn val(this: &Snapshot) -> JsValue;

    #[wasm_bindgen(method, js_name = on)]
    pub fn on(this: &Ref, event: Event, callback: &Closure<dyn FnMut(Snapshot)>);

    #[wasm_bindgen(method, js_name = once)]
    pub fn once(this: &Ref, event: Event, callbacl: &Closure<dyn FnMut(Snapshot)>);

    #[wasm_bindgen(method, js_name = child)]
    pub fn child(this: &Ref, path: String) -> Ref;

    #[wasm_bindgen(method, catch, js_name = get)]
    pub async fn get(this: &Ref) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = set)]
    pub async fn set(this: &Ref, value: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = remove)]
    pub async fn remove(this: &Ref) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = push)]
    pub fn push(this: &Ref) -> Ref;
}
