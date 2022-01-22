#![allow(dead_code)]
use std::time::Duration;

use bevy::{
    core::{Time, Timer},
    prelude::{Res, ResMut},
};

pub(crate) struct GameData {
    pub(crate) round: u32,
    pub(crate) time_before_start: Timer,
    pub(crate) state: GameState,
}

impl Default for GameData {
    fn default() -> Self {
        let timer_before_start = Timer::new(Duration::from_secs(5), false);

        Self {
            round: 0,
            time_before_start: timer_before_start,
            state: GameState::Waiting,
        }
    }
}

pub(crate) enum GameState {
    Paused,
    Playing,
    Waiting,
}

fn gamedata_managing_system(time: Res<Time>, mut gamedata: ResMut<GameData>) {
    if gamedata
        .time_before_start
        .tick(time.delta())
        .just_finished()
    {
        println!("Starting Round");
        gamedata.round += 1;
        gamedata.state = GameState::Playing;
    }
}
