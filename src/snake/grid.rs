use rand;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::constants::{BACKGROUND, GRID_COLS, GRID_ROWS};
use super::segments::Snake;

pub struct Grid {
    pub max_x: usize,
    pub max_y: usize,
    background_color: Color,
}

fn random_color() -> Color {
    Color::RGB(
        (rand::random::<u8>() / 2) + 127,
        (rand::random::<u8>() / 2) + 127,
        (rand::random::<u8>() / 2) + 127,
    )
}

impl Grid {
    pub fn new (n_cols: usize, n_rows: usize) -> Grid {
        let background_color = Color::RGB(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2);
        Grid { max_x: n_cols - 1, max_y: n_rows - 1, background_color: background_color }
    }

    fn draw_cell(&self, renderer: &mut Canvas<Window>, col: u32, row: u32, w: u32, h: u32, color: Color) {
        let x = (col * w) as i32;
        let y = (row * h) as i32;
        let rect = Rect::new(x, y, w, h);
        renderer.set_draw_color(color);
        renderer.fill_rect(rect).expect("draw_cell failed!");
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>, snake: &Snake, food: &(u32, u32)) {
        // let (window_width, window_height) = renderer.window.size;
        let w = 32;
        let h = 32;

        renderer.set_draw_color(self.background_color);
        renderer.clear();

        self.draw_cell(renderer, food.0, food.1, w, h, random_color());

        for segment in &snake.segments {
            self.draw_cell(renderer, segment.0, segment.1, w, h, random_color());
        }

        renderer.present();
    }

    pub fn random_cell_outside (&self, snake: &Snake) -> (u32, u32) {
        if snake.len() >= (GRID_ROWS * GRID_COLS) as usize {
            // field's full of snake, no space for food
            return (0, 0);
        }

        let h = self.max_x as u32;
        let w = self.max_y as u32;
        let cell = (
            rand::random::<u32>() % (h - 1),
            rand::random::<u32>() % (w - 1)
        );

        if snake.segments.contains(&cell) {
            self.random_cell_outside(snake)
        } else {
            cell
        }
    }
}
