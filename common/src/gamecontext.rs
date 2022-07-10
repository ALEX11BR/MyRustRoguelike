use pathfinding::prelude::astar;
use rand::{thread_rng, Rng};
use symmetric_shadowcasting::{compute_fov, Pos};

use crate::{
    being::Being, beingkind::BeingKind, event::Event, level::Level, pickupitem::PickUpItem,
    playeraction::PlayerAction, point::Point, tile::Tile, LEVEL_COUNT, LEVEL_HEIGHT, LEVEL_WIDTH,
};

#[derive(Debug)]
pub struct GameContext {
    pub level: Level,
    pub current_level: i32,
    pub current_turn: u32,
    pub player: Being,
    pub events: Vec<Event>,
}
impl GameContext {
    pub fn new() -> Self {
        let mut context = GameContext {
            level: Level::generate(1),
            current_level: 1,
            current_turn: 1,
            player: Being::new_player(),
            events: vec![],
        };

        context.player.position = context.level.up_stairs;
        context.update_fov();

        context
    }
    fn update_fov(&mut self) {
        compute_fov(
            self.player.position.into(),
            &mut |(y, x): Pos| {
                self.level.tiles[(y as i32, x as i32)].is_blocking()
                    || y == 0
                    || x == 0
                    || (y as i32) == LEVEL_HEIGHT
                    || (x as i32) == LEVEL_WIDTH
            },
            &mut |(y, x): Pos| {
                self.level.last_seen[(y as i32, x as i32)] = self.current_turn;
            },
        );
    }
    pub fn next_turn(&mut self, action: PlayerAction) {
        let mut rng = thread_rng();

        self.events = vec![];
        self.current_turn += 1;

        match action {
            PlayerAction::MoveBy(move_by) => {
                let new_position = self.player.position + move_by;
                if new_position.in_bounds() && self.level.tiles[new_position].is_walkable() {
                    if let Some(enemy) = self
                        .level
                        .enemies
                        .iter_mut()
                        .find(|enemy| enemy.position == new_position)
                    {
                        let damage_dealt = self.player.fight(enemy);
                        self.events.push(Event::Attacked(enemy.kind, damage_dealt));
                    } else {
                        self.player.position = new_position;
                    }
                }
            }
            PlayerAction::Select => {
                if let Tile::Stairs(1) = self.level.tiles[self.player.position] {
                    self.current_level += 1;
                    if self.current_level > LEVEL_COUNT {
                        self.events.push(Event::Won(self.player.experience_points));
                    } else {
                        self.level = Level::generate(self.current_level);
                        self.player.position = self.level.up_stairs;
                        self.update_fov();
                    }

                    return;
                } else if let Tile::Item(item) = self.level.tiles[self.player.position] {
                    match item {
                        PickUpItem::HealthBoost => {
                            self.player.max_health_points += 1;
                            self.player.health_points = self.player.max_health_points;
                        }
                        PickUpItem::AttackBoost => {
                            self.player.max_attack += 1;
                        }
                        PickUpItem::ShieldBoost => {
                            self.player.max_shield += 1;
                        }
                    }
                    self.level.tiles[self.player.position] = Tile::Room;
                }
            }
        }
        self.level.enemies.retain(|enemy| {
            if enemy.health_points > 0 {
                true
            } else {
                self.player.experience_points += enemy.experience_points;
                self.events
                    .push(Event::Killed(enemy.kind, enemy.experience_points));
                false
            }
        });
        self.update_fov();

        // This generates the action of every enemy in the turn. Due to rust borrow rules,
        // you can't have at the same time both
        // a mutable reference to an element of the array of enemies
        // and an immutable one to the whole array.
        // To get around this, I inlined the whole process here,
        // with a for loop based on the index of the enemies in the level's array.
        for i in 0..self.level.enemies.len() {
            match self.level.enemies[i].kind {
                // Regular enemies that chase the player when in sight
                BeingKind::Gnoll | BeingKind::Kestrel | BeingKind::Troll => {
                    if self.level.last_seen[self.level.enemies[i].position] == self.current_turn {
                        if let Some((to_player, _)) = astar(
                            &self.level.enemies[i].position,
                            |&point| {
                                point
                                    .iter_neighbors()
                                    .filter(|&p| self.level.tiles[p].is_walkable())
                                    .filter(|&p| self.level.enemies.iter().all(|e| e.position != p))
                                    .map(|p| (p, 1))
                            },
                            |&Point { y, x }| {
                                x.abs_diff(self.player.position.x)
                                    + y.abs_diff(self.player.position.y)
                            },
                            |&point| point == self.player.position,
                        ) {
                            if to_player[1] == self.player.position {
                                let damage_dealt = self.level.enemies[i].fight(&mut self.player);
                                self.events.push(Event::GotAttacked(
                                    self.level.enemies[i].kind,
                                    damage_dealt,
                                ));
                            } else {
                                self.level.enemies[i].position = to_player[1];
                            }
                        }
                    }
                }
                // Chaotically moving enemies
                BeingKind::Bat | BeingKind::Emu | BeingKind::Zombie => {
                    if self.level.enemies[i]
                        .position
                        .is_neighboring(self.player.position)
                    {
                        let damage_dealt = self.level.enemies[i].fight(&mut self.player);
                        self.events
                            .push(Event::GotAttacked(self.level.enemies[i].kind, damage_dealt));
                    } else {
                        let possible_positions: Vec<Point> = self.level.enemies[i]
                            .position
                            .iter_neighbors()
                            .filter(|&p| self.level.tiles[p].is_walkable())
                            .filter(|&p| self.level.enemies.iter().all(|enemy| enemy.position != p))
                            .collect();
                        if possible_positions.len() > 0 {
                            self.level.enemies[i].position =
                                possible_positions[rng.gen_range(0, possible_positions.len())];
                        }
                    }
                }
                // Static enemies
                BeingKind::AnimatedStatue | BeingKind::LazyImp | BeingKind::StoneSatan => {
                    if self.level.enemies[i]
                        .position
                        .is_neighboring(self.player.position)
                    {
                        let damage_dealt = self.level.enemies[i].fight(&mut self.player);
                        self.events
                            .push(Event::GotAttacked(self.level.enemies[i].kind, damage_dealt));
                    }
                }
                _ => {}
            }
        }

        if self.player.health_points <= 0 {
            self.events.push(Event::Died(self.player.experience_points));
            return;
        }

        match self.level.tiles[self.player.position] {
            Tile::Item(item) => {
                self.events.push(Event::OnItem(Some(item)));
            }
            Tile::Stairs(1) => {
                self.events.push(Event::OnItem(None));
            }
            _ => {}
        }

        if self.current_turn % 10 == 0 {
            self.player.bump_health();
            self.level
                .enemies
                .iter_mut()
                .for_each(|enemy| enemy.bump_health());
        }
    }
}
