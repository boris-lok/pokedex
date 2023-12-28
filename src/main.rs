mod api;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
fn main() {
    api::serve("localhost:8000")
}
