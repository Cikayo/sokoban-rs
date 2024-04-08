use ggez::input::keyboard::KeyCode;
use specs::World;
use std::fmt::{self, Display};
use std::time::Duration;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub enum GameplayState {
    Playing,
    Won,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

// resources.rs
#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
}
