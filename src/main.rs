use std::thread::sleep;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod snake;
use snake::grid::Grid;
use snake::segments::Snake;
use snake::segments::Direction;
use snake::constants::{
    GRID_COLS,
    GRID_ROWS,
    CELL_WIDTH,
    CELL_HEIGHT,
    START_X,
    START_Y,
    MOVEMENT_DELAY,
};

fn main() {
    let (mut canvas, mut events) = snake::init(GRID_COLS * CELL_WIDTH, GRID_ROWS * CELL_HEIGHT);
    let field = Grid::new(GRID_COLS as usize, GRID_ROWS as usize);
    let mut snake = Snake::new(START_X, START_Y);
    let mut food = (START_X - 3, START_Y);

    let mut duration_until_move = Duration::from_millis(MOVEMENT_DELAY);
    let duration_until_draw = Duration::from_millis(33);

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
                let speedup = (snake.segments.len() * 10) as u64;
                food = field.random_cell_outside(&snake);
                duration_until_move = Duration::from_millis(MOVEMENT_DELAY - speedup);
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
