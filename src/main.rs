use bevy::{
    prelude::*,
    sprite::{collide_aabb::{collide, Collision}, Rect},    
    time::FixedTimestep, input::keyboard::KeyboardInput,    
    ui::FocusPolicy,    
};

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 700.0;
const BG_COLOR: Color = Color::rgb(0.20, 0.20, 0.10);

use std::env;

mod main_menu;
mod game;
mod paused;

use main_menu::MainMenuPlugin;
use game::InGamePlugin;
use paused::PausedPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    println!("Program launched!");
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "1");

    App::new()    
        .add_state(GameState::MainMenu)
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Pong!".to_string(),            
            resizable: false,        
            ..default()
        })        
        .insert_resource(ClearColor(BG_COLOR))     
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(MainMenuPlugin)         
        .add_plugin(InGamePlugin)               
        .add_plugin(PausedPlugin)           
        .run();

    println!("Program finished.");
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn pause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,    
    mut state: ResMut<State<GameState>>
) {    
    if keyboard_input.pressed(KeyCode::Space) {
        match state.current() {
            GameState::MainMenu => { /* ... */}
            GameState::InGame => {
                state.push(GameState::Paused).unwrap();
                keyboard_input.reset(KeyCode::Space);
            }
            GameState::Paused => {
                state.pop().unwrap();
                keyboard_input.reset(KeyCode::Space);
            }
        }
    }
}