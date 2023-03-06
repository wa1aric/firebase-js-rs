use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Event {
    ChildAdded = "child_added",
    ChildChanged = "child_changed",
    ChildRemoved = "child_removed",
    ChildMoved = "child_moved",
    Value = "value",
}
