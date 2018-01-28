#![recursion_limit = "1024"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;


mod logger;


use stdweb::web;

fn main() {
    logger::init().expect("failed to initialize logger");
    stdweb::initialize();

    // We can remove this later... but without any log macro call, we would get
    // a warning.
    info!("Logger and stdweb initialized");

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
