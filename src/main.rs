#![recursion_limit = "1024"]

#[macro_use]
extern crate stdweb;

use stdweb::web;

fn main() {
    stdweb::initialize();

    let ref canvas = web::document().get_element_by_id("screen").unwrap();
    let ref ctx = js! { return @{canvas}.getContext("2d"); };
    js! {
        @{ctx}.fillStyle = "rgb("
            + Math.floor(256 * Math.random()) + ", "
            + Math.floor(256 * Math.random()) + ", "
            + Math.floor(256 * Math.random()) + ")";
        @{ctx}.fillRect(0, 0, @{canvas}.width, @{canvas}.height);
    };

    stdweb::event_loop();
}
