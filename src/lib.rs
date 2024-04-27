mod konst;
mod time;
mod utils;

use konst::{WINDOW_HEIGHT, WINDOW_RESIZEABLE, WINDOW_WIDTH};
use macroquad::prelude::*;
use utils::FPSCounter;

// TODO : Put your game resources and other stuff here
pub struct Game {
    fps: FPSCounter,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            fps: FPSCounter::default(),
        }
    }

    pub fn update(&mut self) {
        // let delta = get_frame_time();
        self.fps.update();
    }

    pub fn render(&self) {
        self.fps.draw(None, vec2(32., 32.), 32.);
        // INFO : Ready to use profiler
        // cargo add macroquad-profiler
        // macroquad_profiler::profiler(macroquad_profiler::ProfilerParams {
        //     fps_counter_pos: vec2!(0., 0.),
        // });
    }

    pub async fn run(&mut self) {
        loop {
            self.update();
            self.render();
            next_frame().await;
        }
    }
}

pub fn window() -> Conf {
    Conf {
        window_title: String::from("Mininvader"),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: WINDOW_RESIZEABLE,
        icon: None,
        sample_count: 1,
        high_dpi: true,
        platform: Default::default(),
    }
}
