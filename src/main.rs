use bevy::prelude::*;

mod read;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let inconsolata_handle = asset_server.load("fonts/Inconsolata-Regular.ttf");
    
    commands.spawn((
        Text2d::new("Select a game"),
        TextFont {
            font: inconsolata_handle,
            font_size: 64.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }
    ));
}