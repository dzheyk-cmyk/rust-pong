use bevy::{prelude::*};

use crate::GameState;
use crate::pause_game;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App ) {
        println!("Pausing game!");
        app.
        add_system_set(
            SystemSet::on_update(GameState::Paused)
            .with_system(pause_game)
        );
    }
}

