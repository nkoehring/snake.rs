use std::default::Default;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod config;
mod window;
mod snake;

use config::Config;

use snake::grid::Grid;
use snake::segments::Snake;
use snake::segments::Direction;

fn main() {
    let cfg: Config = Default::default();
    let (mut canvas, mut events) = window::init(cfg.win_width(), cfg.win_height());

    let field = Grid::new(&cfg);
    let mut snake = Snake::new(&cfg);
    let mut food = (cfg.start_x - 3, cfg.start_y);

    let mut duration_until_move = cfg.movement_delay_as_duration();
    let duration_until_redraw = cfg.redraw_delay_as_duration();

    let mut time_since_move = Instant::now();
    let mut time_since_draw = Instant::now();

    'main: loop {
        let now = Instant::now();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => snake.direction = Direction::North,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => snake.direction = Direction::East,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => snake.direction = Direction::South,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => snake.direction = Direction::West,
                _ => continue,
            }
        }

        if time_since_move.elapsed() >= duration_until_move {
            let grow = snake.touches(&food);
            if grow {
                let speedup = snake.len() * 10;

                if speedup < cfg.max_speedup {
                    duration_until_move = Duration::from_millis(cfg.movement_delay - (speedup as u64));
                }

                food = field.random_cell_outside(&snake);
            }

            let crawled = snake.crawl(grow);
            if !crawled {
                println!("Ouch!");
                break 'main;
            }

            time_since_move = Instant::now();
        }

        if time_since_draw.elapsed() >= duration_until_redraw {
            field.draw(&mut canvas, &snake, &food);
            time_since_draw = Instant::now();
        }

        let elapsed = now.elapsed();
        if elapsed < duration_until_redraw {
            sleep(duration_until_redraw - elapsed);
        }
    }
}
