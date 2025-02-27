use std::collections::VecDeque;

use display_info::DisplayInfo;
use mouse_rs::Mouse;

pub trait Pattern {
    fn check_position(&mut self, x: f32, y: f32) -> PatternStatus;
    fn reset_visited(&mut self);
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RectanglePattern {
    display_height: f32,
    display_width: f32,
    start_node: (u8, u8),
    last_visited: (u8, u8),
    to_visit: VecDeque<(u8, u8)>,
    n_horizontal: u8,
    n_vertical: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PatternStatus {
    WrongPattern,
    PatternFinished,
    PatternProgressing,
}

impl RectanglePattern {
    fn new(display_height: f32, display_width: f32) -> Self {
        let n_horizontal = 6;
        let n_vertical = 3;
        let start_node = (0, 0);

        let mut rectangle_pattern = RectanglePattern {
            display_height,
            display_width,
            start_node,
            last_visited: start_node,
            to_visit: VecDeque::new(),
            n_horizontal,
            n_vertical,
        };

        rectangle_pattern.reset_visited();

        rectangle_pattern
    }
}

impl Pattern for RectanglePattern {
    fn reset_visited(&mut self) {
        let mut to_visit = VecDeque::new();

        for i in 1..self.n_horizontal {
            to_visit.push_back((i, 0));
        }

        for i in 1..self.n_vertical {
            to_visit.push_back((self.n_horizontal - 1, i));
        }

        for i in 1..self.n_horizontal {
            to_visit.push_back((self.n_horizontal - i - 1, self.n_vertical - 1));
        }

        for i in 1..self.n_vertical {
            to_visit.push_back((0, self.n_vertical - i - 1));
        }

        self.to_visit = to_visit;
        self.last_visited = self.start_node;
    }

    fn check_position(&mut self, x: f32, y: f32) -> PatternStatus {
        let cell_width = self.display_width / self.n_horizontal as f32;
        let cell_height = self.display_height / self.n_vertical as f32;
        let next_node = self.to_visit.front().unwrap();

        if x >= self.last_visited.0 as f32 * cell_width
            && x <= (self.last_visited.0 + 1) as f32 * cell_width
            && y >= self.last_visited.1 as f32 * cell_height
            && y <= (self.last_visited.1 + 1) as f32 * cell_height
        {
            PatternStatus::PatternProgressing
        } else if x >= next_node.0 as f32 * cell_width
            && x <= (next_node.0 + 1) as f32 * cell_width
            && y >= next_node.1 as f32 * cell_height
            && y <= (next_node.1 + 1) as f32 * cell_height
        {
            self.last_visited = self.to_visit.pop_front().unwrap();

            if self.last_visited == self.start_node {
                PatternStatus::PatternFinished
            } else {
                PatternStatus::PatternProgressing
            }
        } else {
            PatternStatus::WrongPattern
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PatternRecognition<T: Pattern> {
    pattern: T,
}

impl<T: Pattern> PatternRecognition<T> {
    pub fn new_rectangle_pattern() -> PatternRecognition<RectanglePattern> {
        let main_display = DisplayInfo::all()
            .unwrap()
            .into_iter()
            .filter(|display| display.is_primary)
            .collect::<Vec<DisplayInfo>>()
            .pop()
            .unwrap();

        let display_height = main_display.height as f32;
        let display_width = main_display.width as f32;

        let rectangle_pattern = RectanglePattern::new(display_height, display_width);

        PatternRecognition {
            pattern: rectangle_pattern,
        }
    }

    pub fn recognize_pattern(&mut self) -> bool {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(50));

            let current_position = Mouse::new().get_position().unwrap();
            let pattern_status = self
                .pattern
                .check_position(current_position.x as f32, current_position.y as f32);

            match pattern_status {
                PatternStatus::WrongPattern => {
                    self.pattern.reset_visited();
                    return false;
                }
                PatternStatus::PatternFinished => {
                    self.pattern.reset_visited();
                    return true;
                }
                PatternStatus::PatternProgressing => {}
            }
        }
    }
}
