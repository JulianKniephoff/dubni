#![recursion_limit = "1024"]


#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;


use std::rc::Rc;
use stdweb::{
    web::{
        self,
        IEventTarget,
        event::KeyupEvent,
    },
    unstable::TryInto,
};

use rendering::{Color, Renderer};


mod logger;
mod rendering;


fn main_loop(renderer: Rc<Renderer>, _t: f64) {
    renderer.clear(Color {
        r: js! { return Math.random() }.try_into().unwrap(),
        g: js! { return Math.random() }.try_into().unwrap(),
        b: js! { return Math.random() }.try_into().unwrap(),
    });
    web::window().request_animation_frame(|t| main_loop(renderer, t));
}

fn main() {
    logger::init().expect("failed to initialize logger");
    stdweb::initialize();

    // We can remove this later... but without any log macro call, we would get
    // a warning.
    info!("Logger and stdweb initialized");

    let canvas = web::document().get_element_by_id("screen").unwrap();
    let renderer = Rc::new(Renderer::init(canvas));

    let resize = {
        let renderer = renderer.clone();
        move || renderer.resize(
            js! { return window.innerWidth; }.try_into().unwrap(),
            js! { return window.innerHeight; }.try_into().unwrap(),
        )
    };
    resize();
    js! {
        window.addEventListener("resize", () => @{resize()});
    };

    let current_time: f64 = js! { return window.performance.now(); }.try_into().unwrap();
    main_loop(renderer, current_time);

    web::document().add_event_listener(|e: KeyupEvent| info!("{:?}", e));
}
