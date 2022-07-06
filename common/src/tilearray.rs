use std::{
    cmp::{max, min},
    ops::{Index, IndexMut},
};

use rand::random;

use crate::{point::Point, room::Room, tile::Tile, LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct TileArray<T> {
    array: Vec<T>,
}
impl<T: Copy> TileArray<T> {
    pub fn new(default: T) -> Self {
        TileArray {
            array: vec![default; (LEVEL_HEIGHT * LEVEL_WIDTH) as usize],
        }
    }
}
impl TileArray<Tile> {
    pub fn generate_floor_point(&self) -> Point {
        let whole_level = Room::new(0, 0, LEVEL_WIDTH - 1, LEVEL_HEIGHT - 1);
        loop {
            let proposed_position = whole_level.generate_inner_point();
            if self[proposed_position] == Tile::Room {
                break proposed_position;
            }
        }
    }
    pub fn carve_room(&mut self, room: Room) {
        for y in (room.top_left.y + 1)..room.bottom_right.y {
            for x in (room.top_left.x + 1)..room.bottom_right.x {
                self[(y, x)] = Tile::Room;
            }
        }

        if self[room.top_left] == Tile::Room
            && self[(room.top_left.y, room.top_left.x + 1)] == Tile::Wall
            && self[(room.top_left.y + 1, room.top_left.x)] == Tile::Wall
        {
            self[(room.top_left.y + 1, room.top_left.x)] = Tile::Room;
        }
        if self[room.bottom_right] == Tile::Room
            && self[(room.bottom_right.y, room.bottom_right.x - 1)] == Tile::Wall
            && self[(room.bottom_right.y - 1, room.bottom_right.x)] == Tile::Wall
        {
            self[(room.bottom_right.y - 1, room.bottom_right.x)] = Tile::Room;
        }
        if self[(room.bottom_right.y, room.top_left.x)] == Tile::Room
            && self[(room.bottom_right.y, room.top_left.x + 1)] == Tile::Wall
            && self[(room.bottom_right.y - 1, room.top_left.x)] == Tile::Wall
        {
            self[(room.bottom_right.y - 1, room.bottom_right.x)] = Tile::Room;
        }
        if self[(room.top_left.y, room.bottom_right.x)] == Tile::Room
            && self[(room.top_left.y, room.bottom_right.x - 1)] == Tile::Wall
            && self[(room.top_left.y + 1, room.bottom_right.x)] == Tile::Wall
        {
            self[(room.top_left.y + 1, room.bottom_right.x)] = Tile::Room;
        }
    }
    pub fn carve_h_corridor(&mut self, y: i32, x1: i32, x2: i32) {
        for x in min(x1, x2)..(max(x1, x2) + 1) {
            self[(y, x)] = Tile::Room;
        }
    }
    pub fn carve_v_corridor(&mut self, x: i32, y1: i32, y2: i32) {
        for y in min(y1, y2)..(max(y1, y2) + 1) {
            self[(y, x)] = Tile::Room;
        }
    }
    pub fn carve_corridor_between(&mut self, room1: Room, room2: Room) {
        let room1_point = room1.generate_inner_point();
        let room2_point = room2.generate_inner_point();

        if random() {
            self.carve_h_corridor(room1_point.y, room1_point.x, room2_point.x);
            self.carve_v_corridor(room2_point.x, room1_point.y, room2_point.y);
        } else {
            self.carve_v_corridor(room1_point.x, room1_point.y, room2_point.y);
            self.carve_h_corridor(room2_point.y, room1_point.x, room2_point.x);
        }
    }
    pub fn is_pillar_worthy(&self, pillar: Point) -> bool {
        (0..9)
            .map(|i| Point::new(i % 3 - 1, i / 3 - 1))
            .all(|p| self[pillar + p] == Tile::Room)
    }
    pub fn is_door_worthy(&self, door: Point) -> bool {
        self[(door.y - 1, door.x)] == self[(door.y + 1, door.x)]
            && self[(door.y, door.x - 1)] == self[(door.y, door.x + 1)]
            && self[(door.y - 1, door.x)] != self[(door.y, door.x - 1)]
    }
}
impl<T> Index<(i32, i32)> for TileArray<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.array[(index.0 * LEVEL_WIDTH + index.1) as usize]
    }
}
impl<T> IndexMut<(i32, i32)> for TileArray<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        &mut self.array[(index.0 * LEVEL_WIDTH + index.1) as usize]
    }
}
impl<T> Index<Point> for TileArray<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[(index.y, index.x)]
    }
}
impl<T> IndexMut<Point> for TileArray<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[(index.y, index.x)]
    }
}
