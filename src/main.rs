#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    js! {
        console.log("Hello, World!");
    }

    stdweb::event_loop();
}
