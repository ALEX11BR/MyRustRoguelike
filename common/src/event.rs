use crate::{beingkind::BeingKind, pickupitem::PickUpItem, LEVEL_COUNT};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Killed(BeingKind),
    Attacked(BeingKind, i32),
    GotAttacked(BeingKind, i32),
    // OnItem = player is on a Item or a downstairs stair.
    // Used to show a message informing the player that they can press enter to descend/pick up.
    OnItem(Option<PickUpItem>),
    Died,
    Won,
}
impl Event {
    pub fn message(self, level: i32) -> String {
        match self {
            Event::Killed(enemy_kind) => {
                format!("You killed {}.\n", enemy_kind)
            }
            Event::Attacked(enemy_kind, damage) => {
                format!("You attacked {}, dealing {} damage.\n", enemy_kind, damage)
            }
            Event::GotAttacked(enemy_kind, damage) => {
                format!(
                    "You got attacked by {}, taking {} damage.\n",
                    enemy_kind, damage
                )
            }
            Event::OnItem(Some(item)) => {
                format!("Press Enter to apply the {}\n", item)
            }
            Event::OnItem(None) => {
                if level < LEVEL_COUNT {
                    format!(
                        "Press Enter to descend to level {}. You can't go back.\n",
                        level + 1
                    )
                } else {
                    "Press enter to retrieve the Amulet of Yendor, thus winning the game\n"
                        .to_string()
                }
            }
            Event::Died => "YOU LOST THIS GAME...\n".to_string(),
            Event::Won => "YOU WON! THE AMULET OF YENDOR IS YOURS!\n".to_string(),
        }
    }
}
