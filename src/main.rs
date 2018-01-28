#![recursion_limit = "1024"]
#![feature(use_nested_groups)]


#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;


use stdweb::{
    web::{
        self,
        IEventTarget,
        event::KeyupEvent,
    },
    unstable::TryInto,
};


mod logger;


fn draw(ref ctx: stdweb::Value) {
    js! {
        @{ctx}.fillStyle = "rgb("
            + Math.floor(256 * Math.random()) + ", "
            + Math.floor(256 * Math.random()) + ", "
            + Math.floor(256 * Math.random()) + ")";
        @{ctx}.fillRect(0, 0, @{ctx}.canvas.width, @{ctx}.canvas.height);
    }
}

fn main_loop(ctx: stdweb::Value, _t: f64) {
    draw(ctx.clone());
    web::window().request_animation_frame(|t| main_loop(ctx, t));
}

fn main() {
    logger::init().expect("failed to initialize logger");
    stdweb::initialize();

    // We can remove this later... but without any log macro call, we would get
    // a warning.
    info!("Logger and stdweb initialized");

    let ref canvas = web::document().get_element_by_id("screen").unwrap();
    let ctx = js! { return @{canvas}.getContext("2d"); };

    js! {
        const resize = () => {
            @{canvas}.width = window.innerWidth;
            @{canvas}.height = window.innerHeight;
            @{draw}(@{ctx.clone()});
        };
        window.addEventListener("resize", resize);
        resize();
    };

    let current_time: f64 = js! { return window.performance.now(); }.try_into().unwrap();
    main_loop(ctx, current_time);

    web::document().add_event_listener(|e: KeyupEvent| info!("{:?}", e));

    stdweb::event_loop();
}
