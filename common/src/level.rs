use rand::distributions::weighted::alias_method::WeightedIndex;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};

use crate::being::Being;
use crate::beingkind::ENEMIES_KIND;
use crate::pickupitem::PickUpItem;
use crate::point::Point;
use crate::room::Room;
use crate::tile::Tile;
use crate::tilearray::TileArray;

#[derive(Debug)]
pub struct Level {
    pub tiles: TileArray<Tile>,
    pub last_seen: TileArray<u32>,
    pub enemies: Vec<Being>,
    pub up_stairs: Point,
    pub down_stairs: Point,
}
impl Level {
    pub fn generate(level: i32) -> Self {
        let mut rng = thread_rng();

        let mut tiles = TileArray::new(Tile::Wall);

        let first_room = Room::generate();
        tiles.carve_room(first_room);

        let mut prev_room = first_room;
        for _ in 0..10 {
            let new_room = Room::generate();
            tiles.carve_room(new_room);
            tiles.carve_corridor_between(prev_room, new_room);

            prev_room = new_room;
        }

        let last_room = Room::generate_not_overlapping(first_room);
        tiles.carve_room(last_room);
        tiles.carve_corridor_between(prev_room, last_room);

        let up_stairs = first_room.generate_inner_point();
        tiles[up_stairs] = Tile::Stairs(-1);

        let down_stairs = last_room.generate_inner_point();
        tiles[down_stairs] = Tile::Stairs(1);

        for _ in 0..50 {
            let proposed_point = tiles.generate_floor_point();
            if tiles.is_pillar_worthy(proposed_point) {
                tiles[proposed_point] = Tile::Wall;
            } else if tiles.is_door_worthy(proposed_point) {
                tiles[proposed_point] = Tile::Door;
            }
        }

        if rng.gen_bool(0.5) {
            let item_point = tiles.generate_floor_point();
            tiles[item_point] = Tile::Item(PickUpItem::HealthBoost);
        }
        if rng.gen_bool(0.4) {
            let item_point = tiles.generate_floor_point();
            tiles[item_point] = Tile::Item(PickUpItem::AttackBoost);
        }
        if rng.gen_bool(0.4) {
            let item_point = tiles.generate_floor_point();
            tiles[item_point] = Tile::Item(PickUpItem::ShieldBoost);
        }

        let mut enemies = vec![];
        let tier_1_enemy_weight = max(0, min(level + 1, 12 - level));
        let tier_2_enemy_weight = max(0, min(level - 4, 20 - level));
        let tier_3_enemy_weight = max(0, min(level - 12, 26 - level));
        #[rustfmt::skip]
        let enemy_weights = vec![
            tier_1_enemy_weight * 5, tier_1_enemy_weight * 3, tier_1_enemy_weight * 2,
            tier_2_enemy_weight * 5, tier_2_enemy_weight * 3, tier_2_enemy_weight * 2,
            tier_3_enemy_weight * 5, tier_3_enemy_weight * 3, tier_3_enemy_weight * 2,
        ];
        let enemy_dist = WeightedIndex::new(enemy_weights).unwrap();
        for _ in 0..(6 + level / 2) {
            let enemy_position = tiles.generate_floor_point();
            if enemies
                .iter()
                .all(|enemy: &Being| enemy.position != enemy_position)
            {
                enemies.push(Being::new_of_kind(
                    ENEMIES_KIND[enemy_dist.sample(&mut rng)],
                    enemy_position,
                ));
            }
        }

        Level {
            tiles,
            last_seen: TileArray::new(0),
            enemies,
            up_stairs,
            down_stairs,
        }
    }
}
