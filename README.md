# firebase-js-rs

Unofficial Wasm bindings for [Firebase](https://firebase.google.com) JS SDKs written in Rust.

## Get strted

Following example shows how to add email and password sign in to [Sycamore](https://sycamore-rs.netlify.app) app.

### Add and initialize SDK

Install firebase-js-rs by running the following Cargo command in your project directory:

```
cargo add firebase-js-rs
```

or alternatively add the following line to your Cargo.toml:

```toml
firebase-js-rs = "0.1.1"
```

Then add project in the [Firebase console](https://console.firebase.google.com/?authuser=0) and install JS SDKs from the CDN.

```html
<html>
  <head>
    <script src="https://www.gstatic.com/firebasejs/9.17.1/firebase-app-compat.js"></script>
    <script src="https://www.gstatic.com/firebasejs/9.17.1/firebase-auth-compat.js"></script>
  </head>
</html>

```

### Initialize Firebase app and get reference to the authentication service

```rust
use sycamore::prelude::*;
use firebase_js_rs::{app::initialize_app, Config};

fn main() {
  sycamore::render(|cx| {
    view! { cx,
          // Initialize Firebase
          let firebase_app = initialize_app(Config::initialize(
            "api_key", None, None, None, None, None, None,
            ));
          // Get reference to the auth service
            let auth = app.auth();
        }
    });
}

```

### Create new user

```rust
let result = auth.create_user_with_email_and_password(email, password).await;
```

### Sign in user

```rust
let result = auth.sign_in_with_email_and_password(email, password).await;
```

### Observe authentication state

```rust
let callback = Closure::new(move |user: JsValue| {
  // Get info about user
});
auth.on_auth_state_changed(&callback);
callback.forget();
```

## Realtime Database

### Initialize

```rust
let config = Config::initialize({
  "api_key",
  None,
  None,
  Some(String::from("project_id")),
  None,
  None,
  None,
});

// Initialize Firebase
let app = initialize_app(config);

// Initialize Realtime Database
let db = app.db();
```

### Create a new reference

```rust
let ref = db.ref(String::from("/"));
```

### Listen for value events

```rust
let callback = Closure::new(move |snapshot: Snapshot| {
  web_sys::console::log_1(&snapshot.val());
});
r#ref.on(Event::Value, &callback);
callback.forget();
```

## Examples

|Example|Description|
|-|-|
|[sycamore-firebase-auth](https://github.com/wa1aric/firebase-js-rs/tree/main/examples/sycamore-firebase-auth)|Basic Sycamore auth example|
|[leptos-firebase-auth](https://github.com/wa1aric/firebase-js-rs/tree/main/examples/leptos-firebase-auth)|Email and password authentication with Leptos|
