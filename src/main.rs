#![recursion_limit = "1024"]


#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;

mod logger;
mod rendering;
mod game;


use std::rc::Rc;
use stdweb::{
    web::{
        self,
        IEventTarget,
        INonElementParentNode,
        event::KeyUpEvent,
    },
    unstable::TryInto,
};

use rendering::Renderer;
use game::Game;


fn main() {
    logger::init().expect("failed to initialize logger");
    stdweb::initialize();

    // We can remove this later... but without any log macro call, we would get
    // a warning.
    info!("Logger and stdweb initialized");

    let canvas = web::document().get_element_by_id("screen").unwrap();
    let renderer = Rc::new(Renderer::init(canvas.try_into().unwrap()));

    let resize = {
        let renderer = renderer.clone();
        move || renderer.resize(
            web::window().inner_width() as u32,
            web::window().inner_height() as u32,
        )
    };
    resize();
    js! {
        window.addEventListener("resize", () => @{resize()});
    };

    web::document().add_event_listener(|e: KeyUpEvent| info!("{:?}", e));

    let start_time: f64 = js! {
        return window.performance.now();
    }.try_into().unwrap();
    let timestep = 1.0;
    let main_loop = MainLoop::new(Game::new(), start_time, timestep);
    run_main_loop(main_loop, renderer, start_time);
}

fn run_main_loop(
    mut main_loop: MainLoop,
    renderer: Rc<Renderer>,
    current_time: f64
) {
    main_loop.tick(&*renderer, current_time);
    web::window().request_animation_frame(
        |current_time| run_main_loop(main_loop, renderer, current_time)
    );
}

struct MainLoop {
    game: Game,
    timestep: f64,
    last_frame: f64,
    lag: f64,
}

impl MainLoop {
    fn new(game: Game, start_time: f64, timestep: f64) -> Self {
        MainLoop {
            game: game,
            timestep: timestep,
            last_frame: start_time,
            lag: 0.0,
        }
    }

    fn tick(&mut self, renderer: &Renderer, t: f64) {
        let dt = t - self.last_frame;
        self.last_frame = t;
        self.lag += dt;

        while self.lag >= self.timestep {
            self.game.update(self.timestep);
            self.lag -= self.timestep;
        }

        self.game.render(renderer);
    }
}
