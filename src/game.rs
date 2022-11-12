use std::str::FromStr;

use bevy::{
    prelude::*,
    sprite::{collide_aabb::{collide, Collision}, Rect},    
    time::FixedTimestep, input::keyboard::KeyboardInput,    
    ui::FocusPolicy,
};

use crate::GameState;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::BG_COLOR;
use crate::pause_game;

const TIME_STEP: f32 = 1.0 / 60.0;

const RIGHT_WALL: f32 = SCREEN_WIDTH / 2.0;
const LEFT_WALL: f32 = -1.0 * RIGHT_WALL;
const TOP_WALL: f32 = SCREEN_HEIGHT / 2.0;
const BOTTOM_WALL: f32 = -1.0 * TOP_WALL;

const ARENA_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const ARENA_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;

const WALL_THICKNESS: f32 = 100.0;
const WALL_LENGTH: f32 = SCREEN_HEIGHT;

const VERT_OFFSET: f32 = 50.0;
const HORI_OFFSET: f32 = 25.0;

const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(-0.5, 0.5);
const BALL_SPEED: f32 = 450.0;
const BALL_RAD: f32 = 25.0;
const BALL_SIZE: Vec3 = Vec3::new(BALL_RAD,BALL_RAD, 1.0);

const WALL_COLOR: Color = Color::rgb(0.30, 0.30, 0.15);

const PADDLE_OFFSET: f32 = 50.;
const PADDLE_WIDTH: f32 = 15.;
const PADDLE_HEIGHT: f32 = 100.;
const PLAYER_COLOR: Color = Color::BEIGE;
const PLAYER_VELOCITY: f32 = 200.;
const OPPONENT_COLOR: Color = Color::BISQUE;
const OPPONENT_VELOCITY: f32 = 250.;


pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        println!{"Building game!"};
        app
        .insert_resource(Scoreboard {
            player_score: 0,
            opponent_score: 0,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)            
                .with_system(game_setup)                            
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)                                
                .with_system(check_for_collisions)                
                .with_system(move_player.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions))                
                .with_system(move_opponent.before(check_for_collisions))                
                .with_system(update_score)
                .with_system(esc_to_menu)
                .with_system(pause_game)                
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(teardown_ingame)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Paused)
            .with_system(pause_game)            
        );
    }    
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Opponent;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct GameEntity;

#[derive(Component)]
struct Score;

struct OuterBallLoc {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32
}

struct Scoreboard {
    player_score: usize,
    opponent_score: usize,
}

enum Paddle {
    Player,
    Opponent,
}

impl Paddle {
    fn position(&self) -> Vec2 {
        match self {
            Paddle::Player => {
                Vec2::new(WallLocation::Left.inner().x + PADDLE_OFFSET,
                WallLocation::Left.inner().y)
            }
            Paddle::Opponent => {
                Vec2::new(WallLocation::Right.inner().x - PADDLE_OFFSET,
                WallLocation::Left.inner().y)
            }
        }
    }

    fn size(&self) -> Vec2 {
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
    }            
}
#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL + HORI_OFFSET, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL - HORI_OFFSET, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL + VERT_OFFSET),
            WallLocation::Top => Vec2::new(0., TOP_WALL - VERT_OFFSET),
        }
    }

    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, ARENA_HEIGHT + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(ARENA_WIDTH + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }

    // Return the position of the inner surface of a given wall
    fn inner(&self) -> Vec2 {
        match self {
            WallLocation::Top => {
                Vec2::new(WallLocation::Top.position().x,
                 WallLocation::Top.position().y - WALL_THICKNESS/2.)
            }
            WallLocation::Bottom => {
                Vec2::new(WallLocation::Bottom.position().x,
                WallLocation::Bottom.position().y + WALL_THICKNESS/2.)
            }
            WallLocation::Right => {
                Vec2::new(WallLocation::Right.position().x - WALL_THICKNESS/2.,
                 WallLocation::Right.position().y)
            }
            WallLocation::Left => {
                Vec2::new(WallLocation::Left.position().x + WALL_THICKNESS/2.,
                WallLocation::Left.position().y)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),

                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,            
        }
    }
}


fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * TIME_STEP;        
        transform.translation.y += velocity.0.y * TIME_STEP;                        
    }
}

// Use setup function to add entities to the game world
fn game_setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    scoreboard: Res<Scoreboard>,
    state: Res<State<GameState>>) {
    println!("Setting up game!");    
    
   commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: Vec3::new(0.0,0.0,0.0),                
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.5, 0.5),
                ..default()
            },
            ..default()
        })        
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize()*BALL_SPEED))
        .insert(GameEntity);

    // PLayer
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Paddle::Player.size().extend(0.0),
                translation: Paddle::Player.position().extend(0.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider)
        .insert(GameEntity);
        
    // Opponent
    commands
        .spawn()
        .insert(Opponent)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Paddle::Opponent.size().extend(0.0),
                translation: Paddle::Opponent.position().extend(0.0),
                ..default()
            },
            sprite: Sprite {
                color: OPPONENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(Vec2::new(0.0,1.0)*OPPONENT_VELOCITY))
        .insert(Collider)
        .insert(GameEntity);

    // Player Score
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                get_score_str(scoreboard),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    color: PLAYER_COLOR,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::BOTTOM_LEFT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(5.0),
                    left: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Score)
        .insert(GameEntity);
    
    // Spawn walls using implementation of WallBundle enum
    commands.spawn_bundle(
        WallBundle::new(WallLocation::Left)).insert(GameEntity);
    commands.spawn_bundle(
        WallBundle::new(WallLocation::Right)).insert(GameEntity);
    commands.spawn_bundle(
        WallBundle::new(WallLocation::Top)).insert(GameEntity);
    commands.spawn_bundle(
        WallBundle::new(WallLocation::Bottom)).insert(GameEntity);

}


fn get_score_str(scoreboard: Res<Scoreboard>) -> String {    
    format!("Player Score: {}\nComputer Score: {}", 
    scoreboard.player_score, scoreboard.opponent_score)
    .to_string()    
}

fn update_score(
    mut score_query: Query<&mut Text, With<Score>>,
    scoreboard: Res<Scoreboard>
) {
    let mut score = score_query.single_mut();
    score.sections[0].value = get_score_str(scoreboard);    
}

fn is_between(input: f32, val_1: f32, val_2: f32) -> bool {
    if input < val_1 && input > val_2 {
        return true
    }
    false
}

fn check_for_collisions(    
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut opponent_query: Query<&Transform, With<Opponent>>,
    mut scoreboard: ResMut<Scoreboard>
) {    
    let (mut ball_velocity, mut ball_transform) = ball_query.single_mut();
    let player_transform = player_query.single_mut();
    let opponent_transform = opponent_query.single_mut();    

    let ball_side = OuterBallLoc {
        top:    ball_transform.translation.y + ball_transform.scale.y/2.,
        bottom: ball_transform.translation.y - ball_transform.scale.y/2.,
        left:   ball_transform.translation.x - ball_transform.scale.x/2.,
        right:  ball_transform.translation.x + ball_transform.scale.x/2.,
    };

    if is_between(ball_transform.translation.y, 
        player_transform.translation.y + player_transform.scale.y/2.,
        player_transform.translation.y - player_transform.scale.y/2.) &&
        is_between(ball_side.left,
        player_transform.translation.x + player_transform.scale.x/2.,
        player_transform.translation.x - player_transform.scale.x/2.) {
        
            ball_velocity.0.x *= -1.0;
        }

    else if is_between(ball_transform.translation.y, 
        opponent_transform.translation.y + opponent_transform.scale.y/2.,
        opponent_transform.translation.y - opponent_transform.scale.y/2.) &&
        is_between(ball_side.right,
        opponent_transform.translation.x + opponent_transform.scale.x/2.,
        opponent_transform.translation.x - opponent_transform.scale.x/2.) {
        
            ball_velocity.0.x *= -1.0;
        }

    if ball_side.top > WallLocation::Top.inner().y || ball_side.bottom 
        < WallLocation::Bottom.inner().y {
        
        ball_velocity.0.y *= -1.0;
    }

    if ball_side.right > WallLocation::Right.inner().x {        
        scoreboard.opponent_score += 1;              
        ball_velocity.0.x *= -1.0;
    }      

    else if  ball_side.left < WallLocation::Left.inner().x {        
        scoreboard.player_score += 1;  
        ball_velocity.0.x *= -1.0;
    }
         
}


fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        direction = 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction = -1.0; 
    }    

    let new_pos = player_transform.translation.y + direction * PLAYER_VELOCITY * TIME_STEP;

    player_transform.translation.y = new_pos;
}

fn move_opponent(
    mut query: Query<(&mut Velocity, &Transform), With<Opponent>>,
    mut ball_query: Query<&Transform, With<Ball>>
) {
    let (mut opponent_velocity, opponent_transform) 
        = query.single_mut();
    let ypos = opponent_transform.translation.y;
    let ylen = opponent_transform.scale.y;

    let ball_transform = ball_query.single_mut();
    
    if ball_transform.translation.y > ypos {
        opponent_velocity.0.y = OPPONENT_VELOCITY;
    }
    else if ball_transform.translation.y < ypos {
        opponent_velocity.0.y = -1.0*OPPONENT_VELOCITY;
    }
}

fn esc_to_menu(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(GameState::MainMenu)
        .expect("Failed to exit to menu");
        keys.reset(KeyCode::Escape);
    }
}

fn teardown_ingame(
    mut commands: Commands,    
    query: Query<Entity, With<GameEntity>>
) {
    for query_entity in query.iter() {
        commands.entity(query_entity).despawn();
    }
}