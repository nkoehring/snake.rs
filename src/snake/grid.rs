use rand;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::segments::Snake;

pub struct Grid {
    pub cells: Vec<Vec<Color>>,
    pub max_x: usize,
    pub max_y: usize,
}

impl Grid {
    pub fn new (n_cols: usize, n_rows: usize) -> Grid {
        let mut grid_vec = Vec::with_capacity(n_cols as usize);

        for row in 0..n_cols {
            grid_vec.push(Vec::with_capacity(n_rows as usize));

            for _col in 0..n_rows {
                grid_vec[row].push(Color::RGB(35_u8, 15_u8, 13_u8));
            }
        }

        Grid { cells: grid_vec, max_x: n_cols - 1, max_y: n_rows - 1 }
    }

    fn draw_cell(&self, renderer: &mut Canvas<Window>, row: u32, col: u32, w: u32, h: u32) {
        let x = (row * w) as i32;
        let y = (col * h) as i32;
        let rect = Rect::new(x, y, w, h);
        renderer.set_draw_color(self.cells[col as usize][row as usize]);
        renderer.fill_rect(rect).expect("draw_cell failed!");
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>, snake: &Snake, food: &(u32, u32)) {
        // let (window_width, window_height) = renderer.window.size;
        let w = 32;
        let h = 32;

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        self.draw_cell(renderer, food.0, food.1, w, h);

        for segment in &snake.segments {
            self.draw_cell(renderer, segment.0, segment.1, w, h);
        }

        renderer.present();
    }

    pub fn randomize (&mut self) {
        for rows in self.cells.iter_mut() {
            for cell in rows.iter_mut() {
                cell.r = (rand::random::<u8>() / 2) + 127;
                cell.g = (rand::random::<u8>() / 2) + 127;
                cell.b = (rand::random::<u8>() / 2) + 127;
            }
        }
    }

    pub fn random_cell_outside (&self, snake: &Snake) -> (u32, u32) {
        // TODO: think about the case when all fields are full of snake
        let h = self.max_x as u32;
        let w = self.max_y as u32;
        let cell = (
            rand::random::<u32>() % (w - 1),
            rand::random::<u32>() % (h - 1)
        );

        if snake.segments.contains(&cell) {
            self.random_cell_outside(snake)
        } else {
            cell
        }
    }
}
