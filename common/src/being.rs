use std::cmp::max;

use rand::{thread_rng, Rng};

use crate::{beingkind::BeingKind, point::Point};

#[derive(Clone, Debug)]
pub struct Being {
    pub position: Point,
    pub max_health_points: i32,
    pub health_points: i32,
    pub experience_points: i32,
    pub max_attack: i32,
    pub max_shield: i32,
    pub kind: BeingKind,
}
impl Being {
    pub fn new(
        position: Point,
        hp: i32,
        experience_points: i32,
        max_attack: i32,
        max_shield: i32,
        kind: BeingKind,
    ) -> Self {
        Being {
            position,
            max_health_points: hp,
            health_points: hp,
            experience_points,
            max_attack,
            max_shield,
            kind,
        }
    }
    pub fn new_of_kind(kind: BeingKind, position: Point) -> Self {
        let mut rng = thread_rng();

        match kind {
            BeingKind::Player => Being::new(position, 20, 0, 5, 1, kind),

            BeingKind::Gnoll => Being::new(position, 9, rng.gen_range(10, 15), 4, 2, kind),
            BeingKind::Bat => Being::new(position, 10, rng.gen_range(15, 20), 2, 3, kind),
            BeingKind::AnimatedStatue => {
                Being::new(position, 15, rng.gen_range(15, 20), 3, 1, kind)
            }

            BeingKind::Kestrel => Being::new(position, 10, rng.gen_range(20, 25), 5, 5, kind),
            BeingKind::Emu => Being::new(position, 12, rng.gen_range(25, 30), 3, 3, kind),
            BeingKind::LazyImp => Being::new(position, 20, rng.gen_range(25, 30), 5, 1, kind),

            BeingKind::Troll => Being::new(position, 15, rng.gen_range(30, 35), 7, 5, kind),
            BeingKind::Zombie => Being::new(position, 15, rng.gen_range(35, 40), 9, 4, kind),
            BeingKind::StoneSatan => Being::new(position, 30, rng.gen_range(35, 40), 15, 5, kind),
        }
    }
    pub fn new_player() -> Self {
        Being::new_of_kind(
            BeingKind::Player,
            Point::new(0, 0), // dummy position, will be later set to the one of the up_stairs
        )
    }
    pub fn fight(&mut self, other: &mut Being) -> i32 {
        let mut rng = thread_rng();

        let damage_dealt = max(
            0,
            rng.gen_range(0, self.max_attack + 1) - rng.gen_range(0, other.max_shield),
        );
        other.health_points -= damage_dealt;

        damage_dealt
    }
    pub fn bump_health(&mut self) {
        if self.health_points < self.max_health_points {
            self.health_points += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn being_new_player() {
        if let BeingKind::Player = Being::new_player().kind {
        } else {
            panic!();
        }
    }
    #[test]
    fn being_fight() {
        let mut player = Being::new_player();
        let mut enemy = Being::new_of_kind(BeingKind::Bat, Point::new(0, 1));

        let damage_dealt = player.fight(&mut enemy);

        assert_eq!(player.health_points, player.max_health_points);
        assert_eq!(enemy.health_points + damage_dealt, enemy.max_health_points);
    }
    #[test]
    fn being_bump_health() {
        let mut enemy = Being::new_of_kind(BeingKind::Bat, Point::new(1, 1));

        enemy.bump_health();
        assert_eq!(enemy.health_points, enemy.max_health_points);

        enemy.health_points = 1;
        enemy.bump_health();
        assert_eq!(enemy.health_points, 2);
    }
}
