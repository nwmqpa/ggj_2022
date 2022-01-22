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

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component, Debug)]
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
#[derive(Component, Debug)]
pub struct PlayerMovement {
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32,
    pub scale: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            up: Default::default(),
            down: Default::default(),
            left: Default::default(),
            right: Default::default(),
            scale: 50.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Self(100.0)
    }
}