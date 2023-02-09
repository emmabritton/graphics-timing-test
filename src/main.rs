use std::collections::VecDeque;
use std::time::Instant;

use color_eyre::Result;
use pixels_graphics_lib::buffer_graphics_lib::color::{BLACK, DARK_GRAY, LIGHT_GRAY, RED, WHITE, YELLOW};
use pixels_graphics_lib::buffer_graphics_lib::drawable::{Drawable, stroke};
use pixels_graphics_lib::buffer_graphics_lib::shapes::CreateDrawable;
use pixels_graphics_lib::buffer_graphics_lib::text::format::Positioning::{LeftBottom, LeftTop, RightBottom, RightTop};
use pixels_graphics_lib::buffer_graphics_lib::text::pos::TextPos;
use pixels_graphics_lib::buffer_graphics_lib::text::TextSize::Normal;
use pixels_graphics_lib::graphics_shapes::coord::Coord;
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::prelude::*;

const HEIGHT: usize = 240;
const WIDTH: usize = 240;
const HISTORY_SIZE: usize = 120;
const SECS_PER_DEGREE: f64 = 0.0027778;

fn main() -> Result<()> {
    let system = Box::new(TimingTest::new());
    run(WIDTH, HEIGHT, "Timing Test", system, Options {ups:120,..Options::default()})?;
    Ok(())
}

#[derive(Debug)]
struct TimingTest {
    start: Instant,
    last_delta: f64,
    delta_history: Vec<f64>,
    highest_delta: f64,
    seconds_since_start: f32,
    ticks: usize,
    draws: usize,
    degrees: f64,
    next_degree: f64
}


impl TimingTest {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            last_delta: 0.0,
            delta_history: vec!(0.0; HISTORY_SIZE),
            highest_delta: 0.0,
            seconds_since_start: 0.0,
            ticks: 0,
            draws: 0,
            degrees: 0.0,
            next_degree: SECS_PER_DEGREE
        }
    }
}

impl System for TimingTest {
    fn window_prefs(&self) -> Option<WindowPreferences> {
        Some(WindowPreferences::new("app","emmabritton","timing-test").unwrap())
    }

    fn update(&mut self, timing: &Timing) {
        self.last_delta = timing.delta;
        self.delta_history.remove(0);
        self.delta_history.push(self.last_delta);
        self.highest_delta = self.highest_delta.max(timing.delta);
        self.ticks = timing.updates;
        self.draws = timing.renders;
        self.seconds_since_start = timing.now.duration_since(timing.started_at).as_secs_f32();
        self.next_degree -= timing.fixed_time_step;
        while self.next_degree < 0.0 {
            self.next_degree += SECS_PER_DEGREE;
            self.degrees += 1.0;
            if self.degrees >= 360.0 {
                self.degrees = 0.0
            }
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        graphics.clear(BLACK);
        let graph_height = 60.0;
        let graph_top = HEIGHT - (graph_height as usize);
        graphics.draw_text(&format!("Ticks: {: <6}", self.ticks), TextPos::px((1, 1)), (WHITE, Normal, LeftTop));
        graphics.draw_text(&format!("Draws: {: <6}", self.draws), TextPos::px((1, 10)), (WHITE, Normal, LeftTop));
        graphics.draw_text(&format!("Secs: {: >5.2}", self.seconds_since_start), TextPos::px((WIDTH - 1, 1)), (WHITE, Normal, RightTop));
        graphics.draw_text(&format!("Highest: {:0.4}", self.highest_delta), TextPos::px((1, graph_top - 4)), (WHITE, Normal, LeftBottom));
        graphics.draw_text(&format!("Delta: {:0.4}", self.last_delta), TextPos::px((WIDTH - 1, graph_top -4)), (WHITE, Normal, RightBottom));
        graphics.draw_text(&format!("{:0.4}", self.next_degree), TextPos::px((WIDTH/2 + 54, HEIGHT/2-41)), (WHITE, Normal, LeftBottom));
        graphics.draw_text(&format!("{: >3}°", self.degrees), TextPos::px((WIDTH/2 + 54, HEIGHT/2-39)), (WHITE, Normal, LeftTop));

        let center = Coord::new((WIDTH/2) as isize, (HEIGHT/2 - 40) as isize);
        graphics.draw_circle(Circle::new(center, 50), stroke(DARK_GRAY));
        let line = Line::new(center, center - (0, 50))
            .rotate_around(self.degrees as isize, center);
        let drawable = Drawable::from_obj(line, stroke(LIGHT_GRAY));
        graphics.draw(&drawable);

        let step = WIDTH / HISTORY_SIZE;

        for (i,nums) in self.delta_history.windows(2).enumerate() {
            let x1 = i * step;
            let y1 = ((nums[0] / self.highest_delta) * graph_height) as usize;
            let x2 = (i +1) * step;
            let y2 = ((nums[1] / self.highest_delta) * graph_height) as usize;
            graphics.draw_line((x1, HEIGHT - y1),(x2,HEIGHT - y2), RED);
        }
    }
}