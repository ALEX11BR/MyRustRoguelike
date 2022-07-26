use common::event::Event;
use common::gamecontext::GameContext;
use common::playeraction::PlayerAction;
use common::point::Point;
use common::{LEVEL_HEIGHT, LEVEL_WIDTH};
use gloo_events::{EventListener, EventListenerOptions};
use gloo_utils::window;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::gencell::{gen_cell, gen_empty_cell, gen_enemy_cell, gen_player_cell};

mod gencell;

struct App {
    game_context: GameContext,
    end_game_event: Option<Event>,
    key_listener: Option<EventListener>,
}
impl Component for App {
    type Message = PlayerAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_context: GameContext::new(),
            end_game_event: None,
            key_listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.game_context.next_turn(msg);

        if let Some(event) = self.game_context.events.iter().find(|event| match event {
            Event::Died(_) | Event::Won(_) => true,
            _ => false,
        }) {
            self.end_game_event.replace(*event);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(event) = self.end_game_event {
            return html! {
                <div>
                    <p>{event.message(self.game_context.current_level)}</p>
                    <p>{"You can start the game again reloading the page."}</p>
                </div>
            };
        }

        let mut events_contents = vec![];
        for event in &self.game_context.events {
            events_contents.push(html! {
                <li>{event.message(self.game_context.current_level)}</li>
            });
        }

        let link = ctx.link();

        let level_string = format!("Level {}", self.game_context.current_level);
        let turn_string = format!("Turn {}", self.game_context.current_turn);
        let hp_string = format!(
            "HP: {}/{}",
            self.game_context.player.health_points, self.game_context.player.max_health_points
        );
        let xp_string = format!("XP: {}", self.game_context.player.experience_points);
        let attack_string = format!("Attack: 0-{}", self.game_context.player.max_attack);
        let shield_string = format!("Shield: 0-{}", self.game_context.player.max_shield);

        let table_contents = (0..LEVEL_HEIGHT).map(|y| {
            let table_row = (0..LEVEL_WIDTH).map(|x| {
                let position = Point::new(x, y);

                if self.game_context.level.last_seen[position] > 0 {
                    if position == self.game_context.player.position {
                        gen_player_cell()
                    } else if self.game_context.level.last_seen[position]
                        == self.game_context.current_turn
                    {
                        if let Some(enemy) = self
                            .game_context
                            .level
                            .enemies
                            .iter()
                            .find(|enemy| enemy.position == position)
                        {
                            gen_enemy_cell(enemy.kind)
                        } else {
                            gen_cell(
                                self.game_context.level.tiles[position],
                                true,
                                self.game_context.current_level,
                            )
                        }
                    } else {
                        gen_cell(
                            self.game_context.level.tiles[position],
                            false,
                            self.game_context.current_level,
                        )
                    }
                } else {
                    gen_empty_cell()
                }
            });

            html! {
                <tr>{ for table_row }</tr>
            }
        });

        let select_callback = link.callback(|_| PlayerAction::Select);
        let skip_turn_callback = link.callback(|_| PlayerAction::MoveBy(Point::new(0, 0)));

        let move_left_callback = link.callback(|_| PlayerAction::MoveBy(Point::new(-1, 0)));
        let move_down_callback = link.callback(|_| PlayerAction::MoveBy(Point::new(0, 1)));
        let move_up_callback = link.callback(|_| PlayerAction::MoveBy(Point::new(0, -1)));
        let move_right_callback = link.callback(|_| PlayerAction::MoveBy(Point::new(1, 0)));

        html! {
            <div id="root">
                <table id="level_display">{ for table_contents }</table>
                <div id="stats_display">
                    <p>{level_string}</p>
                    <p>{turn_string}</p>
                    <p id="hp_display">{hp_string}</p>
                    <p id="xp_display">{xp_string}</p>
                    <p id="attack_display">{attack_string}</p>
                    <p id="shield_display">{shield_string}</p>
                    <ul id="events_display">{ for events_contents }</ul>
                </div>
                <table id="controls">
                    <tr>
                        <td><button onclick={skip_turn_callback}>{"Skip turn"}</button></td>
                        <td><button onclick={move_up_callback}>{"↑"}</button></td>
                        <td><button onclick={select_callback}>{"Enter"}</button></td>
                    </tr>
                    <tr>
                        <td><button onclick={move_left_callback}>{"←"}</button></td>
                        <td><button onclick={move_down_callback}>{"↓"}</button></td>
                        <td><button onclick={move_right_callback}>{"→"}</button></td>
                    </tr>
                </table>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link();

            let select_callback = link.callback(|()| PlayerAction::Select);
            let skip_turn_callback = link.callback(|()| PlayerAction::MoveBy(Point::new(0, 0)));

            let move_left_callback = link.callback(|()| PlayerAction::MoveBy(Point::new(-1, 0)));
            let move_down_callback = link.callback(|()| PlayerAction::MoveBy(Point::new(0, 1)));
            let move_up_callback = link.callback(|()| PlayerAction::MoveBy(Point::new(0, -1)));
            let move_right_callback = link.callback(|()| PlayerAction::MoveBy(Point::new(1, 0)));

            self.key_listener.replace(EventListener::new_with_options(
                &window(),
                "keydown",
                EventListenerOptions::run_in_capture_phase(),
                move |event| {
                    if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
                        match event.key().as_str() {
                            "ArrowLeft" | "h" | "H" => {
                                move_left_callback.emit(());
                            }
                            "ArrowDown" | "j" | "J" => {
                                move_down_callback.emit(());
                            }
                            "ArrowUp" | "k" | "K" => {
                                move_up_callback.emit(());
                            }
                            "ArrowRight" | "l" | "L" => {
                                move_right_callback.emit(());
                            }
                            " " => {
                                skip_turn_callback.emit(());
                            }
                            "Enter" => {
                                select_callback.emit(());
                            }
                            _ => {}
                        }
                    }
                },
            ));
        } else if let Some(_) = self.end_game_event {
            self.key_listener.take();
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
