use bevy::{
    prelude::*,
    app::AppExit,
};


use crate::GameState;

pub struct MainMenuPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct StartGameButton;

#[derive(Component)]
struct QuitButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        println!{"Building main menu!"};
        app
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
            .with_system(button_system)
            .with_system(start_button_sys)
            .with_system(quit_button_sys)                    
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
            .with_system(teardown_main_menu)
        );
    }
}

fn teardown_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<Button>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<
    (&Interaction, &mut UiColor, &Children),
    (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {                
                *color = Color::rgb(0.9,0.9,0.1).into();
            }
            Interaction::Hovered => {                
                *color = Color::rgb(0.9,0.1,0.1).into();
            }
            Interaction::None => {                
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn start_button_sys(
    mut interaction_query: Query<
    &Interaction, With<StartGameButton>>,
    mut state: ResMut<State<GameState>>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked    => {                
                state.set(GameState::InGame).expect("Failed to enter game");                
            }
            Interaction::Hovered    => { }
            Interaction::None       => { }
        }
    }
}

fn quit_button_sys(
    mut interaction_query: Query<
        &Interaction, With<QuitButton>>,
    mut exit: EventWriter<AppExit>   
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked    => {
                exit.send(AppExit);
            }
            Interaction::Hovered    => { }
            Interaction::None       => { }
        }
    }
}


fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {    
    commands                
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })        
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start Game",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(StartGameButton);

        // Quit Button
        commands                
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })        
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Quit Game",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(QuitButton);
}