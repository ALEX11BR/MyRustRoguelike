use common::event::Event::{Attacked, Died, GotAttacked, Won};
use common::gamecontext::GameContext;
use common::pickupitem::PickUpItem;
use common::playeraction::PlayerAction;
use common::point::Point;
use common::tile::Tile;
use common::{LEVEL_HEIGHT, LEVEL_WIDTH};
use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode::Blend;
use sdl2::rwops::RWops;

use crate::appstate::AppState;
use crate::rendertext::{render_char, render_text};

mod appstate;
mod rendertext;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;
pub const TILE_WIDTH: i32 = 15;
pub const TILE_HEIGHT: i32 = 30;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let sdl_video = sdl_context.video().unwrap();
    let window = sdl_video
        .window("MyRustRoguelike", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_blend_mode(Blend);
    let texture_creator = canvas.texture_creator();

    let sdl_ttf = sdl2::ttf::init().unwrap();
    let font_rwops = RWops::from_bytes(include_bytes!("iosevka-custom-regular.ttf")).unwrap();
    let font = sdl_ttf.load_font_from_rwops(font_rwops, 30).unwrap();

    let sdl_controller = sdl_context.game_controller().unwrap();
    let mut controller_0 = sdl_controller.open(0).ok();

    let mut context = GameContext::new();
    let mut app_state = AppState::InGame;

    'game: loop {
        if let AppState::ShowingEnd(event) = app_state {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { .. }
                    | Event::ControllerButtonDown { .. } => {
                        break 'game;
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            let mut game_window = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

            let _ = render_text(
                &format!(
                    "{}{}",
                    event.message(context.current_level),
                    "Press any key to exit the game..."
                ),
                Color::WHITE,
                &mut game_window,
                &font,
                &mut canvas,
                &texture_creator,
            );

            canvas.present();

            continue 'game;
        }

        let mut context_has_changed = false;

        for event in event_pump.poll_iter() {
            let context_already_changed = context_has_changed;
            context_has_changed = true;
            match event {
                Event::Quit { .. } => {
                    break 'game;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::DPadUp,
                    ..
                } => {
                    context.next_turn(PlayerAction::MoveBy(Point::new(0, -1)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::DPadDown,
                    ..
                } => {
                    context.next_turn(PlayerAction::MoveBy(Point::new(0, 1)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::DPadLeft,
                    ..
                } => {
                    context.next_turn(PlayerAction::MoveBy(Point::new(-1, 0)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::DPadRight,
                    ..
                } => {
                    context.next_turn(PlayerAction::MoveBy(Point::new(1, 0)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::X, ..
                } => {
                    context.next_turn(PlayerAction::MoveBy(Point::new(0, 0)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::KpEnter | Keycode::Return),
                    ..
                }
                | Event::ControllerButtonDown {
                    button: Button::A, ..
                } => {
                    context.next_turn(PlayerAction::Select);
                }
                _ => {
                    context_has_changed = context_already_changed;
                }
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.draw_line(
            (TILE_WIDTH * LEVEL_WIDTH, 0),
            (TILE_WIDTH * LEVEL_WIDTH, SCREEN_HEIGHT as i32),
        );

        let mut info_window = Rect::new(
            TILE_WIDTH * LEVEL_WIDTH + 1,
            0,
            SCREEN_WIDTH - (TILE_WIDTH * LEVEL_WIDTH) as u32 - 1,
            SCREEN_HEIGHT,
        );

        let _ = render_text(
            &format!(
                "Level {}\nTurn {}",
                context.current_level, context.current_turn
            ),
            Color::WHITE,
            &mut info_window,
            &font,
            &mut canvas,
            &texture_creator,
        );
        let _ = render_text(
            &format!(
                "HP: {}/{}",
                context.player.health_points, context.player.max_health_points
            ),
            Color::RED,
            &mut info_window,
            &font,
            &mut canvas,
            &texture_creator,
        );
        let _ = render_text(
            &format!("XP: {}", context.player.experience_points),
            Color::GREEN,
            &mut info_window,
            &font,
            &mut canvas,
            &texture_creator,
        );
        let _ = render_text(
            &format!("Attack: 0-{}", context.player.max_attack),
            Color::MAGENTA,
            &mut info_window,
            &font,
            &mut canvas,
            &texture_creator,
        );
        let _ = render_text(
            &format!("Shielding: 0-{}", context.player.max_shield),
            Color::CYAN,
            &mut info_window,
            &font,
            &mut canvas,
            &texture_creator,
        );

        for event in &context.events {
            if let GotAttacked(_, _) | Attacked(_, _) = event {
                if context_has_changed {
                    if let Some(controller) = &mut controller_0 {
                        let _ = controller.set_rumble(0xFFFF, 0x8FFF, 333);
                    }
                }
            }
            match event {
                Died(_) | Won(_) => {
                    if let Some(controller) = &mut controller_0 {
                        let _ = controller.set_rumble(0x8FFF, 0xFFFF, 666);
                    }

                    app_state = AppState::ShowingEnd(*event);

                    continue 'game;
                }
                _ => {
                    let _ = render_text(
                        &format!("* {}", event.message(context.current_level)),
                        Color::WHITE,
                        &mut info_window,
                        &font,
                        &mut canvas,
                        &texture_creator,
                    );
                }
            }
        }

        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                let (char, mut fg_color, mut bg_color) = match context.level.tiles[(y, x)] {
                    Tile::Room => ('.', Color::WHITE, Color::BLACK),
                    Tile::Item(PickUpItem::HealthBoost) => ('+', Color::RED, Color::BLACK),
                    Tile::Item(PickUpItem::AttackBoost) => ('/', Color::MAGENTA, Color::BLACK),
                    Tile::Item(PickUpItem::ShieldBoost) => ('[', Color::CYAN, Color::BLACK),
                    Tile::Wall => (' ', Color::BLACK, Color::WHITE),
                    Tile::Door => ('+', Color::BLACK, Color::WHITE),
                    Tile::Stairs(-1) => ('<', Color::WHITE, Color::BLACK),
                    Tile::Stairs(1) => (
                        if context.current_level < 25 { '>' } else { '0' },
                        Color::WHITE,
                        Color::BLACK,
                    ),
                    _ => (' ', Color::WHITE, Color::BLACK),
                };
                if context.level.last_seen[(y, x)] < context.current_turn {
                    fg_color.a = 100;
                    bg_color.a = 100;
                }
                if context.level.last_seen[(y, x)] > 0 {
                    let _ = render_char(
                        char,
                        fg_color,
                        bg_color,
                        Point::new(x, y),
                        &font,
                        &mut canvas,
                        &texture_creator,
                    );
                }
            }
        }

        for enemy in &context.level.enemies {
            if context.level.last_seen[enemy.position] == context.current_turn {
                if let Some(char) = enemy.kind.to_string().chars().next() {
                    let _ = render_char(
                        char,
                        Color::WHITE,
                        Color::BLACK,
                        enemy.position,
                        &font,
                        &mut canvas,
                        &texture_creator,
                    );
                }
            }
        }

        let _ = render_char(
            '@',
            Color::GREEN,
            Color::BLACK,
            context.player.position,
            &font,
            &mut canvas,
            &texture_creator,
        );

        canvas.present();
    }
}
