#[allow(warnings)]
mod bindings;
#[allow(warnings)]
mod fermyon;
#[allow(warnings)]
mod wasi;

struct Component;

bindings::export!(Component with_types_in bindings);
