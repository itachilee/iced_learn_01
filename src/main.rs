use bevy::prelude::*;


#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);
fn main() {
    App::new()
    .add_plugins((DefaultPlugins,HelloPlugin))
    // .add_plugins(DefaultPlugins)

        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());



    let texture = asset_server.load("character.png");
    commands.spawn(SpriteBundle{
        sprite: Sprite { custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default() 
      },
      texture,
        ..default()
    
    });
}


fn add_people(mut commands: Commands) {

    println!("adding person...");

    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}



#[derive(Resource)]
struct GreetTimer(Timer);


fn greet_people(time: Res<Time>,mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {

    if timer.0.tick(time.delta()).just_finished(){
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
   
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app 
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
       
        .add_systems(Startup,add_people)
        .add_systems(Startup,setup)
        //   . add_startup_system(add_people)
            .add_systems(Update,greet_people);
    }
}