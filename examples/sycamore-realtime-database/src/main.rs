use firebase_js_rs::{app::initialize_app, database::Snapshot, Closure, Config};
use sycamore::{futures::spawn_local_scoped, prelude::*};
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
    let r#ref = db.r#ref(String::from("/"));
    let callback = Closure::new(move |snapshot: Snapshot| {
        web_sys::console::log_1(&snapshot.val());
    });
    r#ref.on(String::from("value"), &callback);
    callback.forget();

    sycamore::render(|cx| {
        view! {
            cx,
            button(on:click = move |_| {
                let ref_clone = r#ref.clone();
                spawn_local_scoped(cx, async move {
                    let res = ref_clone.get().await;
                    let snapshot = Snapshot::from(res.unwrap());
                    log_1(&snapshot.val());
                });
            }) { "Get" }
        }
    })
}
