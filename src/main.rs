use std::thread::sleep;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod snake;
use snake::grid::Grid;
use snake::segments::Snake;
use snake::segments::Direction;

fn main() {
    let (mut canvas, mut events) = snake::init(1024, 768);
    let mut field = Grid::new(32, 24);
    let mut snake = Snake::new(16, 12);
    let mut food = (10_u32, 10_u32);

    let duration_until_randomize = Duration::from_millis(200);
    let duration_until_move = Duration::from_millis(200);
    let duration_until_draw = Duration::from_millis(33);

    let mut time_since_randomize = Instant::now();
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

        if time_since_randomize.elapsed() >= duration_until_randomize {
            field.randomize();
            time_since_randomize = Instant::now();
        }

        if time_since_move.elapsed() >= duration_until_move {
            let grow = snake.touches(&food);
            if grow {
                food = field.random_cell_outside(&snake);
            }

            let crawled = snake.crawl(grow);
            if !crawled {
                println!("Ouch!");
                break 'main;
            }

            time_since_move = Instant::now();
        }

        if time_since_draw.elapsed() >= duration_until_draw {
            field.draw(&mut canvas, &snake, &food);
            time_since_draw = Instant::now();
        }

        let elapsed = now.elapsed();
        if elapsed < duration_until_draw {
            sleep(duration_until_draw - elapsed);
        }
    }
}
