use firebase_js_rs::{Closure, Config};
use leptos::{wasm_bindgen::JsValue, *};
use web_sys::console::log_1;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let api_key = std::env!("FIREBASE_API_KEY");
    let (firebase_app, _) = create_signal(
        cx,
        firebase_js_rs::app::initialize_app(Config::initialize(
            api_key, None, None, None, None, None, None,
        ))
        .unwrap(),
    );
    let (user, set_user) = create_signal(cx, JsValue::NULL);
    let (email, set_email) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let callback = Closure::new(move |user: JsValue| {
        log_1(&user);
        set_user.set(user);
    });
    firebase_app.get().auth().on_auth_state_changed(&callback);
    callback.forget();
    let sign_in = move |_| {
        spawn_local(async move {
            let _ = firebase_app
                .get()
                .auth()
                .sign_in_with_email_and_password(email.get(), password.get())
                .await;
        });
    };
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = firebase_app.get().auth().sign_out().await;
        });
    };
    view! {
        cx,
        <h1>"Leptos Firebase Auth"</h1>
        {
            move || user.get().is_null().then(|| view! {
                cx,
                <input
                    type = "email"
                    prop:value = { move || email.get() }
                    on:input =  move |event| set_email.set(event_target_value(&event)) />
                <input
                    type="password"
                    prop:value = { move || password.get() }
                    on:input = move |event| set_password.set(event_target_value(&event)) />
                <button on:click = sign_in>"Sign In"</button>
            })
        }
        {
            move || user.get().is_object().then(|| view! {
                cx,
                <button on:click = sign_out>"Sign Out"</button>
            })
        }
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App /> })
}
