use std::collections::VecDeque;
use crate::config::Config;

pub enum Direction { North, East, South, West }
pub struct Snake {
    pub segments: VecDeque<(u32, u32)>,
    pub direction: Direction,
    bound_x: u32,
    bound_y: u32,
}

impl Snake {
    pub fn new (cfg: &Config) -> Snake {
        let mut snake = Snake {
            segments: VecDeque::new(),
            direction: Direction::North,
            bound_x: cfg.grid_cols,
            bound_y: cfg.grid_rows
        };
        snake.segments.push_front(cfg.start_as_coords());
        snake
    }

    pub fn len (&self) -> usize {
        self.segments.len()
    }

    pub fn touches(&self, cell: &(u32, u32)) -> bool {
        self.segments.contains(cell)
    }

    pub fn crawl (&mut self, grow: bool) -> bool {
        let new_head = match self.direction {
            Direction::North => self.head_north(),
            Direction::East => self.head_east(),
            Direction::South => self.head_south(),
            Direction::West => self.head_west(),
        };

        if self.segments.contains(&new_head) {
            return false;
        }

        self.segments.push_front(new_head);
        if !grow {
            self.segments.pop_back();
        }

        true
    }

    fn head_north (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        if old_head.1 > 0 {
            (old_head.0, old_head.1 - 1)
        } else {
            (old_head.0, self.bound_y - 1)
        }
    }

    fn head_east (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        ((old_head.0 + 1) % self.bound_x, old_head.1)
    }

    fn head_south (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        (old_head.0, (old_head.1 + 1) % self.bound_y)
    }

    fn head_west (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        if old_head.0 > 0 {
            (old_head.0 - 1, old_head.1)
        } else {
            (self.bound_x - 1, old_head.1)
        }
    }
}
