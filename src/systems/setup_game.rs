#![allow(dead_code)]
use bevy::prelude::{Commands, Component};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
enum Role {
    Defender,
    Assailant,
}

pub fn setup_game(commands: &mut Commands) {
    commands
        .spawn()
        .insert(Position::new(10., 10.))
        .insert(Name("Player 1".to_string()))
        .insert(Role::Defender);

    commands
        .spawn()
        .insert(Position::new(10., 20.))
        .insert(Name("Player 2".to_string()))
        .insert(Role::Assailant);
}
