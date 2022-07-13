use crate::{beingkind::BeingKind, pickupitem::PickUpItem, LEVEL_COUNT};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Killed(BeingKind, i32 /* xp gained */),
    Attacked(BeingKind, i32 /* damage dealt */),
    GotAttacked(BeingKind, i32 /*damage dealt */),
    // OnItem = player is on a Item or a downstairs stair.
    // Used to show a message informing the player that they can press enter to descend/pick up.
    OnItem(Option<PickUpItem>),
    Died(i32 /* xp on death */),
    Won(i32 /* xp on win */),
}
impl Event {
    pub fn message(self, level: i32) -> String {
        match self {
            Event::Killed(enemy_kind, xp) => {
                format!("You killed {}, gaining {} XP.\n", enemy_kind, xp)
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
            Event::Died(xp) => format!("YOU LOST THIS GAME...\nYou died with {} XP.\n", xp),
            Event::Won(xp) => format!(
                "YOU WON! THE AMULET OF YENDOR IS YOURS!\nYou won with {} XP.\n",
                xp
            ),
        }
    }
}
