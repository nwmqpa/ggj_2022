use bevy::prelude::Component;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub enum Role {
    Defender,
    Assailant,
}
