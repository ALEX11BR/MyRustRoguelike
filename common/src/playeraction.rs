use crate::point::Point;

#[derive(Clone, Copy, Debug)]
pub enum PlayerAction {
    MoveBy(Point),
    Select,
}
