use std::path::Path;

use bevy::prelude::*;
use bevy_cobweb_ui::{prelude::*, sickle::{UiContextRoot, UiRoot}};

use crate::{GameState, GAME_PATH};

#[derive(States, Default, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum MenuState {
    GameSelect,
    FileSelect,
    Reading,
    #[default]
    Disabled,
}

#[derive(Component)]
enum Game {
    FNaF1,
}

impl Game {
    fn value(&self) -> &str {
        match *self {
            Game::FNaF1 => "fnaf1",
        }
    }
}

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .load("ui/game_select.cob")
        .load("ui/file_select.cob")
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::GameSelect), game_select_setup)
        .add_systems(OnEnter(MenuState::FileSelect), file_select_setup);
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::GameSelect);
}

#[derive(Component)]
struct GameSelect;

fn game_select_setup(mut commands: Commands, mut scene_loader: ResMut<SceneLoader>, mut menu_state: ResMut<NextState<MenuState>>) {
    commands.ui_root().load_scene_and_edit(("ui/game_select.cob", "root"), &mut scene_loader, |root| {
        root.edit("buttons", |buttons| {
            buttons.edit("fnaf1", |fnaf1| {
                fnaf1.on_pressed(move |mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>| {
                    select_game(commands, menu_state, Game::FNaF1);
                });
            });
        });
    });
}

fn select_game(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>, game: Game) {
    let exists = Path::new(&format!("{}{}", GAME_PATH, game.value())).exists();

    commands.spawn(game);

    if exists {

    } else {
        menu_state.set(MenuState::FileSelect);
    }
}

fn file_select_setup(mut commands: Commands, mut scene_loader: ResMut<SceneLoader>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
    
    commands.ui_root().load_scene(("ui/file_select.cob", "root"), &mut scene_loader);
}