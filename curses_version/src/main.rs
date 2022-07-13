use common::{
    event::Event, gamecontext::GameContext, pickupitem::PickUpItem, playeraction::PlayerAction,
    point::Point, tile::Tile, LEVEL_HEIGHT, LEVEL_WIDTH,
};
use pancurses::{
    chtype, endwin, init_pair, initscr, newwin, noecho, start_color, Input, COLOR_BLACK,
    COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_PAIR, COLOR_RED, COLOR_WHITE,
};

const HP_COLOR: chtype = 1;
const ATTACK_COLOR: chtype = 2;
const SHIELD_COLOR: chtype = 3;
const WALL_BACKGROUND: chtype = 4;
const HP_BACKGROUND: chtype = 5;
const XP_COLOR: chtype = 6;

fn main() {
    initscr();
    let game_window = newwin(LEVEL_HEIGHT, LEVEL_WIDTH, 0, 0);
    let info_frame = newwin(LEVEL_HEIGHT, 0, 0, LEVEL_WIDTH);
    let info_window = info_frame
        .derwin(LEVEL_HEIGHT - 2, info_frame.get_max_x() - 2, 1, 1)
        .unwrap();

    noecho();
    start_color();
    game_window.keypad(true);
    info_frame.draw_box(0, 0);
    info_frame.refresh();

    init_pair(HP_COLOR as i16, COLOR_RED, COLOR_BLACK);
    init_pair(ATTACK_COLOR as i16, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(SHIELD_COLOR as i16, COLOR_CYAN, COLOR_BLACK);
    init_pair(WALL_BACKGROUND as i16, COLOR_BLACK, COLOR_WHITE);
    init_pair(HP_BACKGROUND as i16, COLOR_BLACK, COLOR_RED);
    init_pair(XP_COLOR as i16, COLOR_GREEN, COLOR_BLACK);

    let mut context = GameContext::new();

    'game: loop {
        info_window.clear();
        game_window.clear();

        info_window.attron(COLOR_PAIR(HP_BACKGROUND));
        info_window.mvaddstr(
            0,
            0,
            format!(
                "HP: {}/{}\n",
                context.player.health_points, context.player.max_health_points
            ),
        );
        info_window.attroff(COLOR_PAIR(HP_BACKGROUND));

        info_window.attron(COLOR_PAIR(XP_COLOR));
        info_window.addstr(format!("XP: {}\n", context.player.experience_points));
        info_window.attroff(COLOR_PAIR(XP_COLOR));

        info_window.attron(COLOR_PAIR(ATTACK_COLOR));
        info_window.addstr(format!("Attack: 0-{}\n", context.player.max_attack));
        info_window.attroff(COLOR_PAIR(ATTACK_COLOR));

        info_window.attron(COLOR_PAIR(SHIELD_COLOR));
        info_window.addstr(format!("Shielding: 0-{}\n\n", context.player.max_shield));
        info_window.attroff(COLOR_PAIR(SHIELD_COLOR));

        info_window.addstr(format!(
            "Level {}\nTurn {}\n\n",
            context.current_level, context.current_turn
        ));

        for event in &context.events {
            match event {
                Event::Died(_) | Event::Won(_) => {
                    game_window.mvaddstr(0, 0, event.message(context.current_level));
                    game_window.addstr("Press any key to exit the game...");
                    game_window.refresh();
                    info_window.refresh();

                    game_window.getch();
                    break 'game;
                }
                _ => {
                    info_window.addstr(format!("* {}", event.message(context.current_level)));
                }
            }
        }

        game_window.mv(0, 0);
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                match (context.level.last_seen[(y, x)], context.level.tiles[(y, x)]) {
                    (t, Tile::Room) if t == context.current_turn => {
                        game_window.addch('.');
                    }
                    (t, Tile::Item(item)) if t > 0 => match item {
                        PickUpItem::HealthBoost => {
                            game_window.attron(COLOR_PAIR(HP_COLOR));
                            game_window.addch('+');
                            game_window.attroff(COLOR_PAIR(HP_COLOR));
                        }
                        PickUpItem::AttackBoost => {
                            game_window.attron(COLOR_PAIR(ATTACK_COLOR));
                            game_window.addch('/');
                            game_window.attroff(COLOR_PAIR(ATTACK_COLOR));
                        }
                        PickUpItem::ShieldBoost => {
                            game_window.attron(COLOR_PAIR(SHIELD_COLOR));
                            game_window.addch('[');
                            game_window.attroff(COLOR_PAIR(SHIELD_COLOR));
                        }
                    },
                    (t, Tile::Wall) if t > 0 => {
                        game_window.attron(COLOR_PAIR(WALL_BACKGROUND));
                        game_window.addch(' ');
                        game_window.attroff(COLOR_PAIR(WALL_BACKGROUND));
                    }
                    (t, Tile::Door) if t > 0 => {
                        game_window.attron(COLOR_PAIR(WALL_BACKGROUND));
                        game_window.addch('+');
                        game_window.attroff(COLOR_PAIR(WALL_BACKGROUND));
                    }
                    (_, Tile::Stairs(-1)) => {
                        game_window.addch('<');
                    }
                    (t, Tile::Stairs(1)) if t > 0 => {
                        game_window.addch(if context.current_level < 25 { '>' } else { '0' });
                    }
                    _ => {
                        game_window.addch(' ');
                    }
                };
            }
        }
        for enemy in &context.level.enemies {
            if context.level.last_seen[enemy.position] == context.current_turn {
                if let Some(char) = enemy.kind.to_string().chars().next() {
                    game_window.mvaddch(enemy.position.y, enemy.position.x, char);
                }
            }
        }
        game_window.mvaddch(context.player.position.y, context.player.position.x, '@');
        game_window.mv(context.player.position.y, context.player.position.x);

        info_window.refresh();
        game_window.refresh();

        match game_window.getch() {
            Some(Input::Character('q')) => {
                game_window.clear();
                game_window.mvaddstr(0, 0, "Are you sure you want to leave the game?\n");
                game_window.addstr("Any progress will be lost! [y/N]:");
                game_window.refresh();

                if let Some(Input::Character('y' | 'Y')) = game_window.getch() {
                    break 'game;
                }
            }
            Some(Input::KeyUp | Input::Character('k')) => {
                context.next_turn(PlayerAction::MoveBy(Point::new(0, -1)));
            }
            Some(Input::KeyDown | Input::Character('j')) => {
                context.next_turn(PlayerAction::MoveBy(Point::new(0, 1)));
            }
            Some(Input::KeyLeft | Input::Character('h')) => {
                context.next_turn(PlayerAction::MoveBy(Point::new(-1, 0)));
            }
            Some(Input::KeyRight | Input::Character('l')) => {
                context.next_turn(PlayerAction::MoveBy(Point::new(1, 0)));
            }
            Some(Input::Character(' ')) => {
                context.next_turn(PlayerAction::MoveBy(Point::new(0, 0)));
            }
            Some(Input::KeyEnter | Input::Character('\n')) => {
                context.next_turn(PlayerAction::Select);
            }
            _ => {}
        }
    }
    endwin();
}
