use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::toolkit::{rand_rgb, rand_coord};
use super::segments::Snake;
use crate::config::Config;

pub struct Grid {
    cell_width: u32,
    cell_height: u32,
    size: usize,
    max_x: usize,
    max_y: usize,
    background_color: Color,
}

impl Grid {
    pub fn new (cfg: &Config) -> Grid {
        Grid {
            cell_width: cfg.cell_width,
            cell_height: cfg.cell_height,
            size: (cfg.grid_rows * cfg.grid_cols) as usize,
            max_x: (cfg.grid_cols - 1) as usize,
            max_y: (cfg.grid_rows - 1) as usize,
            background_color: cfg.background_color_as_rgb(),
        }
    }

    fn draw_cell(&self, renderer: &mut Canvas<Window>, col: u32, row: u32, w: u32, h: u32, color: Color) {
        let x = (col * w) as i32;
        let y = (row * h) as i32;
        let rect = Rect::new(x, y, w, h);
        renderer.set_draw_color(color);
        renderer.fill_rect(rect).expect("draw_cell failed!");
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>, snake: &Snake, food: &(u32, u32)) {
        let w = self.cell_width;
        let h = self.cell_height;

        renderer.set_draw_color(self.background_color);
        renderer.clear();

        self.draw_cell(renderer, food.0, food.1, w, h, rand_rgb());

        for segment in &snake.segments {
            self.draw_cell(renderer, segment.0, segment.1, w, h, rand_rgb());
        }

        renderer.present();
    }

    pub fn random_cell_outside (&self, snake: &Snake) -> (u32, u32) {
        if snake.len() >=  self.size {
            // field's full of snake, no space for food
            return (0, 0);
        }

        let h = (self.max_x - 1) as u32;
        let w = (self.max_y - 1) as u32;
        rand_coord(&(h, w), &|cell| !snake.touches(cell))
    }
}
