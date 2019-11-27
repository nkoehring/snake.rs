use std::collections::VecDeque;

pub enum Direction { North, East, South, West }
pub struct Snake {
    pub segments: VecDeque<(u32, u32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new (x: u32, y: u32) -> Snake {
        let mut snake = Snake {
            segments: VecDeque::new(),
            direction: Direction::North,
        };
        snake.segments.push_front((x, y));
        snake
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
        (old_head.0, old_head.1 - 1)
    }

    fn head_east (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        (old_head.0 + 1, old_head.1)
    }

    fn head_south (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        (old_head.0, old_head.1 + 1)
    }

    fn head_west (&mut self) -> (u32, u32) {
        let old_head = self.segments.front().expect("never empty");
        (old_head.0 - 1, old_head.1)
    }
}
