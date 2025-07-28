use bevy::prelude::*;

#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub enum AppState {
    Join,
    MainMenu,
    Game,
}
#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub enum GameState {
    Running,
    Paused,
    SettingsMenu,
}
#[derive(States,Debug,Hash,Eq,PartialEq,Clone)]
pub enum LoadingState {
    Loading,
    FadeOut,
    FadeIn,
    Abrupt,
    Ready,
}