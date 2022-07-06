use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickUpItem {
    HealthBoost,
    AttackBoost,
    ShieldBoost,
}
impl Display for PickUpItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PickUpItem::HealthBoost => "Health Boost",
                PickUpItem::AttackBoost => "Attack Boost",
                PickUpItem::ShieldBoost => "Shield Boost",
            }
        )
    }
}
