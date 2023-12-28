mod domain;
mod repositories;
mod api;

#[macro_use]
extern crate rouille;
fn main() {
    api::serve("localhost:8000")
}
