use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main(){
 App::new()
 .add_plugins(DefaultPlugins)
 .add_systems(Startup, spawn_player)
 .add_systems(Startup, spawn_camera)

 
 .run();
}

#[derive(Component)]
pub struct Player{}


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: ResMut<AssetServer>,
){
    let window =window_query.get_single().unwrap();

    let texture = asset_server.load("sprites/tile_0084.png");
    commands.spawn((
        SpriteBundle{
            transform: Transform::from_xyz(window.width() /2. , window.height() /2., 0.),
            texture: texture,
            ..default()
        },
        Player{}
    ));
}



pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() /2. , window.height() /2., 0.),
         ..default()
    });
}


pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform,With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transform) =player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;


    }


}