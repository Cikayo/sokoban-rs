use crate::components::{Position, Renderable};
use crate::constants::TILE_WIDTH;
use crate::resources::Gameplay;
use ggez::graphics::{Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment};
use ggez::Context;
use glam::Vec2;
use specs::{prelude::*, System};

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
}
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        let mut canvas = Canvas::from_frame(self.context, Color::WHITE);

        // Draw code here...
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image =
                Image::from_path(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
        }

        self.draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);

        canvas.finish(self.context);
    }
}
