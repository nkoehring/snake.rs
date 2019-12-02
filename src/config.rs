use std::time::Duration;
use sdl2::pixels::Color;

pub struct Config {
    pub cell_width: u32,
    pub cell_height: u32,
    pub grid_cols: u32,
    pub grid_rows: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub redraw_delay: u64,
    pub movement_delay: u64,
    pub max_speedup: usize,
    pub background_color: (u8, u8, u8),
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cell_width: 32,
            cell_height: 32,
            grid_cols: 32,
            grid_rows: 24,
            start_x: 16,
            start_y: 12,
            redraw_delay: 33,
            movement_delay: 300,
            max_speedup: 250,
            background_color: (0, 0, 0),
        }
    }
}

impl Config {
    pub fn win_width(&self) -> u32 {
        self.grid_cols * self.cell_width
    }
    pub fn win_height(&self) -> u32 {
        self.grid_rows * self.cell_height
    }
    pub fn movement_delay_as_duration(&self) -> Duration {
        Duration::from_millis(self.movement_delay)
    }
    pub fn redraw_delay_as_duration(&self) -> Duration {
        Duration::from_millis(self.redraw_delay)
    }
    pub fn background_color_as_rgb(&self) -> Color {
        let bg = self.background_color;
        Color::RGB(bg.0, bg.1, bg.2)
    }
    pub fn start_as_coords(&self) -> (u32, u32) {
        (self.start_x, self.start_y)
    }
}
