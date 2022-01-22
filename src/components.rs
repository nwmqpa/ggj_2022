use bevy::prelude::Component;

#[derive(Component, Debug)]
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

impl Role {
    /// Returns `true` if the role is [`Assailant`].
    ///
    /// [`Assailant`]: Role::Assailant
    pub fn is_assailant(&self) -> bool {
        matches!(self, Self::Assailant)
    }
}

#[derive(Component)]
pub struct Enemy;
