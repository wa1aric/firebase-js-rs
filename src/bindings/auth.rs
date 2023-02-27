use wasm_bindgen::prelude::*;
use crate::app::FirebaseApp;

#[wasm_bindgen]
extern "C" {
    pub type Auth;

    #[wasm_bindgen(method, js_name = auth)]
    pub fn auth(_: &FirebaseApp) -> Auth;

    #[wasm_bindgen(method, js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(this: &Auth, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(method, catch, js_name = createUserWithEmailAndPassword)]
    pub async fn create_user_with_email_and_password(this: &Auth, email: String, password: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signInWithEmailAndPassword)]
    pub async fn sign_in_with_email_and_password(
        this: &Auth,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signOut)]
    pub async fn sign_out(this: &Auth) -> Result<JsValue, JsValue>;
}
