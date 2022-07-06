use rand::{thread_rng, Rng};

use crate::{point::Point, LEVEL_HEIGHT, LEVEL_WIDTH, MAX_ROOM_WIDTH, MIN_ROOM_WIDTH};

#[derive(Clone, Copy, Debug)]
pub struct Room {
    pub top_left: Point,
    pub bottom_right: Point,
}
impl Room {
    pub fn new(top_left_x: i32, top_left_y: i32, width: i32, height: i32) -> Self {
        Room {
            top_left: Point::new(top_left_x, top_left_y),
            bottom_right: Point::new(top_left_x + width, top_left_y + height),
        }
    }
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        let width = rng.gen_range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH);
        let height = rng.gen_range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH);
        let top_left_x = rng.gen_range(0, LEVEL_WIDTH - width);
        let top_left_y = rng.gen_range(0, LEVEL_HEIGHT - height);

        Room::new(top_left_x, top_left_y, width, height)
    }
    pub fn generate_not_overlapping(room: Room) -> Self {
        loop {
            let proposed = Room::generate();
            if !room.overlaps(proposed) {
                break proposed;
            }
        }
    }
    pub fn generate_inner_point(self) -> Point {
        let mut rng = thread_rng();

        Point::new(
            rng.gen_range(self.top_left.x + 1, self.bottom_right.x),
            rng.gen_range(self.top_left.y + 1, self.bottom_right.y),
        )
    }
    pub fn overlaps(self, other: Room) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.top_left.y < other.bottom_right.y
            && other.top_left.x < self.bottom_right.x
            && other.top_left.y < self.bottom_right.y
    }
}
