mod config;

use raylib::prelude::*;
use config::Config;
use rand::{distributions::Uniform, prelude::*};

fn main() {
    let config = Config::read_config();
    let mut builder = raylib::init();
    builder.title("Breakout");
    builder.size(config.width.into(), config.height.into());

    if config.vsync {
        builder.vsync();
    }

    let (mut rl, thread) = builder.build();

    if let Some(fps) = config.fps_cap {
        rl.set_target_fps(fps.into());
    }

    rl.set_exit_key(None);

    let mut x = 0;
    let mut y = 0;
    let mut vel_x = 300;
    let mut vel_y = 300;
    let mut color = Color::WHITE;
    let text = "Hello world!";
    let font_size = 20;
    let text_width = rl.begin_drawing(&thread).measure_text(text, font_size);
    let mut rng = rand::thread_rng();
    let color_dist = Uniform::from(0..255);
    let mut random_color = || {
        let r = color_dist.sample(&mut rng);
        let g = color_dist.sample(&mut rng);
        let b = color_dist.sample(&mut rng);
        Color::new(r, g, b, 255)
    };

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text(text, x, y, font_size, color);

        if x < 0 || x + text_width > config.width.into() {
            vel_x = -vel_x;
            color = random_color();
        }

        if y < 0 || y + font_size > config.height.into() {
            vel_y = -vel_y;
            color = random_color();
        }

        x += (vel_x as f32 * dt).round() as i32;
        y += (vel_y as f32 * dt).round() as i32;
    }
}
