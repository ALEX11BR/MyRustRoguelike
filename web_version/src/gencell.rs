use common::beingkind::BeingKind;
use common::pickupitem::PickUpItem;
use common::tile::Tile;
use yew::{classes, html, Html};

pub fn gen_player_cell() -> Html {
    html! {
        <td class="player" title="You">{"@"}</td>
    }
}

pub fn gen_empty_cell() -> Html {
    html! {
        <td>{"\u{00a0}"}</td>
    }
}

pub fn gen_enemy_cell(kind: BeingKind) -> Html {
    html! {
        <td title={kind.to_string()}>
            {kind.to_string().chars().next().unwrap_or('?')}
        </td>
    }
}

pub fn gen_cell(tile: Tile, in_fov: bool, level: i32) -> Html {
    html! {
        <td title={tile.get_name()} class={classes!(
            if in_fov {None} else {Some("out_of_fov")},
            if tile == Tile::Door || tile == Tile::Wall {Some("door_or_wall")} else {None},
            if tile == Tile::Item(PickUpItem::HealthBoost) {Some("health_boost")} else {None},
            if tile == Tile::Item(PickUpItem::AttackBoost) {Some("attack_boost")} else {None},
            if tile == Tile::Item(PickUpItem::ShieldBoost) {Some("shield_boost")} else {None},
        )}>
            {match tile {
                Tile::Room => ".",
                Tile::Item(PickUpItem::HealthBoost) => "+",
                Tile::Item(PickUpItem::AttackBoost) => "/",
                Tile::Item(PickUpItem::ShieldBoost) => "[",
                Tile::Wall => "\u{00a0}",
                Tile::Door => "+",
                Tile::Stairs(-1) => "<",
                Tile::Stairs(1) => if level < 25 {">"} else {"0"},
                _ => "\u{00a0}"
            }}
        </td>
    }
}
