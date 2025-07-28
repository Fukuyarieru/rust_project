use bevy::prelude::*;

#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub(crate) enum AppState {
    Join,
    MainMenu,
    Game,
}
#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub(crate) enum GameState {
    Running,
    Paused,
    SettingsMenu,
}
#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub(crate) enum LoadingState {
    Loading,
    FadeOut,
    FadeIn,
    Abrupt,
    Ready,
}