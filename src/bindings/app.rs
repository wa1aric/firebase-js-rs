use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type FirebaseApp;

    #[wasm_bindgen(catch, js_namespace = ["firebase"], js_name = initializeApp)]
    pub fn initialize_app(config: JsValue) -> Result<FirebaseApp, JsValue>;
}
