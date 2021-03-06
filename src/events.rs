#![allow(dead_code)]
use bevy::prelude::{Entity, EventReader, ResMut};

use crate::gamestate::GameData;

/// EndRoundEvent is sent when the loose (or win) condition is reached in the current round.
pub struct EndRoundEvent {
    kind: EndRoundKind,
}

pub enum EndRoundKind {
    Success(f32),
    Failed(FailureKind),
}

pub enum FailureKind {
    Timer(u32),
    DeadPlayer(Entity),
}

fn end_round_event_listener(
    mut events: EventReader<EndRoundEvent>,
    mut gamedata: ResMut<GameData>,
) {
    for event in events.iter() {
        match &event.kind {
            EndRoundKind::Failed(failure_kind) => {
                gamedata.time_before_start.reset();
                match failure_kind {
                    FailureKind::Timer(monsters_left) => {
                        println!("Failed with {monsters_left} monster left.")
                    }
                    FailureKind::DeadPlayer(_entity) => {
                        println!("Failed because a player is dead.")
                    }
                }
            }
            EndRoundKind::Success(time_left) => println!("You win with {time_left} seconds left."),
        }
    }
}

/// StartRoundEvent is sent when the Timer started by the listenner of the `EndRoundEvent` ends.
pub struct StartRoundEvent;

fn start_round_event(mut events: EventReader<StartRoundEvent>) {
    for _event in events.iter() {
        // Set the GameState to Playing
        // Invert Player Roles
        invert_player_roles();
        // Setup Timers and Mobs to spawn
        setup_round();
    }
}

fn setup_round() {
    todo!()
}

fn invert_player_roles() {
    todo!()
}
