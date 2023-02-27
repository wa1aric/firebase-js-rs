use firebase_js_rs::{
    app::{initialize_app, FirebaseApp},
    Closure, Config,
};
use sycamore::{futures::spawn_local_scoped, prelude::*, rt::JsValue};

struct Session {
    pub user: RcSignal<JsValue>,
}

impl Session {
    pub fn set_user(&self, user: JsValue) {
        self.user.set(user);
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let session_signal = create_rc_signal(Session {
        user: create_rc_signal(JsValue::NULL),
    });
    let session = session_signal.clone(); // Need to figure out how to not to clone
    provide_context(cx, session_signal);

    let api_key = std::env!("FIREBASE_API_KEY");
    let firebase_app = create_rc_signal(
        initialize_app(Config::initialize(
            api_key, None, None, None, None, None, None,
        ))
        .unwrap(),
    );
    provide_context(cx, firebase_app);

    let callback = Closure::new(move |user: JsValue| {
        session.get().set_user(user);
    });
    use_context::<RcSignal<FirebaseApp>>(cx)
        .get()
        .auth()
        .on_auth_state_changed(&callback);
    callback.forget();

    let email = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());

    view! { cx,
        h1 { "Sycamore Firebase Auth" }
        div {
            (
                if use_context::<RcSignal<Session>>(cx).get().user.get().is_null() {
                    view! { cx,
                        div {
                            input(bind:value=email, type="email", placeholder="Email")
                            input(bind:value=password, type="password", placeholder="Password")
                            button(on:click=move |_| {
                                spawn_local_scoped(cx, async move {
                                    let _ = use_context::<RcSignal<FirebaseApp>>(cx).get().auth().sign_in_with_email_and_password(email.to_string(), password.to_string()).await;
                                });
                            }) { "Sign In" }
                        }
                    }
                }
                else {
                    view! { cx,
                        div {
                            button(on:click=move |_| {
                                spawn_local_scoped(cx, async move {
                                    let _ = use_context::<RcSignal<FirebaseApp>>(cx).get().auth().sign_out().await;
                                })
                            }) { "Sign Out" }
                        }
                    }
                }
            )
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            
        }
    });
}
