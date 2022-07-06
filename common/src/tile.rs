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
}
