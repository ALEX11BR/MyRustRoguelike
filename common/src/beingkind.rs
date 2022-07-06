use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum BeingKind {
    Player,
    AnimatedStatue,
    Bat,
    Emu,
    Gnoll,
    Kestrel,
    LazyImp,
    StoneSatan,
    Troll,
    Zombie,
}
impl Display for BeingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BeingKind::Player => "Player",
                BeingKind::AnimatedStatue => "Animated Statue",
                BeingKind::Bat => "Bat",
                BeingKind::LazyImp => "Lazy Imp",
                BeingKind::Emu => "Emu",
                BeingKind::Gnoll => "Gnoll",
                BeingKind::Kestrel => "Kestrel",
                BeingKind::StoneSatan => "Stone Satan",
                BeingKind::Troll => "Troll",
                BeingKind::Zombie => "Zombie",
            }
        )
    }
}

#[rustfmt::skip]
pub const ENEMIES_KIND: [BeingKind; 9] = [
    BeingKind::Gnoll,   BeingKind::Bat,    BeingKind::AnimatedStatue,
    BeingKind::Kestrel, BeingKind::Emu,    BeingKind::LazyImp,
    BeingKind::Troll,   BeingKind::Zombie, BeingKind::StoneSatan,
];
