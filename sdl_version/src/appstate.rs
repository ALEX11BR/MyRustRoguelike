use common::event::Event;

#[derive(Clone, Copy, Debug)]
pub enum AppState {
    InGame,
    ShowingEnd(Event),
}