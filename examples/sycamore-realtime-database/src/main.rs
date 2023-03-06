use firebase_js_rs::{
    app::initialize_app,
    database::{Ref, Snapshot},
    Closure, Config,
};
use sycamore::{futures::spawn_local_scoped, prelude::*, rt::JsValue};
use web_sys::console::log_1;

fn main() {
    let api_key = std::env!("API_KEY");
    let project_id = std::env!("PROJECT_ID");
    let app = initialize_app(Config::initialize(
        api_key,
        None,
        None,
        Some(String::from(project_id)),
        None,
        None,
        None,
    ))
    .unwrap();
    let db = app.database();

    sycamore::render(|cx| {
        let r#ref = create_rc_signal(db.r#ref(String::from("/")));
        let callback = Closure::new(move |snapshot: Snapshot| {
            web_sys::console::log_1(&snapshot.val());
        });
        r#ref.get().on(String::from("value"), &callback);
        provide_context(cx, r#ref);
        callback.forget();
        view! {
            cx,
            button(on:click = move |_| {
                spawn_local_scoped(cx, async move {
                    let res = use_context::<RcSignal<Ref>>(cx).get().get().await;
                    let snapshot = Snapshot::from(res.unwrap());
                    log_1(&snapshot.val());
                });
            }) { "Get" }
            button(on:click = move |_| {
                spawn_local_scoped(cx, async move {
                    use_context::<RcSignal<Ref>>(cx).get().set(JsValue::NULL).await;
                });
            }) { "Set" }
        }
    })
}
