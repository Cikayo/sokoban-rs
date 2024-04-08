use std::path;

use ggez::event::{self};
use ggez::input::keyboard::KeyInput;
use ggez::{conf, Context, GameResult};
use specs::{prelude::*, World};

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        {
            let mut rs = RenderingSystem { context: ctx };
            // rs.run(&self.world);
            rs.run_now(&self.world);
        }

        Ok(())
    }
    fn key_down_event(
        &mut self,
        _context: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> Result<(), ggez::GameError> {
        println!("Key pressed: {:?}", input.keycode);
        let mut input_queue = self.world.write_resource::<InputQueue>();
        match input.keycode {
            Some(k) => {
                input_queue.keys_pressed.push(k);
            }
            _ => (),
        };

        Ok(())
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rs_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (ctx, event_loop) = context_builder.build()?;
    let state = Game { world };
    event::run(ctx, event_loop, state)
}
