use crate::components::{Position, Renderable};
use crate::constants::TILE_WIDTH;
use crate::resources::{Gameplay, Time};
use ggez::graphics::{Canvas, Color, DrawParam, Image, InstanceArray, PxScale, Text, TextFragment};

use ggez::timer;
use ggez::Context;
use glam::Vec2;
use itertools::Itertools;
use specs::{prelude::*, System};
use std::collections::HashMap;
use std::time::Duration;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
        let text = Text::new(TextFragment {
            text: text_string.to_string(),
            color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            scale: Some(PxScale::from(20.0)),
            ..Default::default()
        });

        let draw_params = Vec2::new(x, y);

        canvas.draw(&text, draw_params);
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        // let path_index = match renderable.kind() {
        //     RenderableKind::Static => {
        //         // We only have one image, so we just return that
        //         0
        //     }
        //     RenderableKind::Animated => {
        //         // If we have multiple, we want to select the right one based on the delta time.
        //         // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
        //         // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
        //         // we technically are on the next iteration of the loop (or on 0), but we will let
        //         // the renderable handle this logic of wrapping frames.
        //         ((delta.as_millis() % 1000) / 250) as usize
        //     }
        // };
        // renderable.path(path_index)
        renderable.path.clone()
    }
}
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        let mut canvas = Canvas::from_frame(self.context, Color::WHITE);

        // Draw code here...
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            // let image =
            //     Image::from_path(self.context, renderable.path.clone()).expect("expected image");

            let image_path = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;

            // draw
            let draw_param = DrawParam::new().dest(Vec2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_param);
            // canvas.draw(&image, draw_params);
        }

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::from_path(self.context, image_path).expect("expected image");

                let mut sprite_batch = InstanceArray::new(self.context, image);

                for draw_param in draw_params.iter() {
                    sprite_batch.push(*draw_param);
                }
                canvas.draw(&sprite_batch, DrawParam::new());
            }
        }

        self.draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);
        let fps = format!("FPS: {:.0}", timer::fps(self.context));
        self.draw_text(&mut canvas, &fps, 525.0, 120.0);

        canvas.finish(self.context).expect("game error");
    }
}
