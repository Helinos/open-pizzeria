use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

mod read;
mod menu;

const GAME_PATH: &str = "data/";

#[derive(States, Default, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Waiting,
    Menu,
    FNaF1,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, CobwebUiPlugin))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(LoadState::Done), menu_setup)
        .add_plugins(menu::menu_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn menu_setup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Menu);
}