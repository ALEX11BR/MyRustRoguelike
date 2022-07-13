use crate::pickupitem::PickUpItem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Room,
    // Stairs(level_progression) = stairs that lead to level current_level + level_progression
    Stairs(i32),
    Door,
    Item(PickUpItem),
}
impl Tile {
    pub fn is_walkable(self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Room | Tile::Stairs(_) | Tile::Door | Tile::Item(_) => true,
        }
    }
    pub fn is_blocking(self) -> bool {
        match self {
            Tile::Wall | Tile::Door => true,
            Tile::Room | Tile::Stairs(_) | Tile::Item(_) => false,
        }
    }
    pub fn get_name(self) -> String {
        match self {
            Tile::Room => "Room".to_string(),
            Tile::Item(PickUpItem::HealthBoost) => "Health boost".to_string(),
            Tile::Item(PickUpItem::AttackBoost) => "Attack boost".to_string(),
            Tile::Item(PickUpItem::ShieldBoost) => "Shield boost".to_string(),
            Tile::Wall => "Wall".to_string(),
            Tile::Door => "Door".to_string(),
            Tile::Stairs(_) => "Stairs".to_string(),
        }
    }
}
