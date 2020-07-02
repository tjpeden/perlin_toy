use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::{ButtonArgs, RenderArgs, UpdateArgs};

use crate::noise_map::NoiseMap;

pub struct Application {
    gl: GlGraphics,
    width: u32,
    height: u32,
    noise_map: NoiseMap,
}

impl Application {
    pub fn new(width: u32, height: u32, gl: GlGraphics) -> Self {
        let noise_map = NoiseMap::new(width as usize, height as usize, 0.3);

        Application {
            gl,
            width,
            height,
            noise_map,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use crate::colors::*;
        use graphics::*;

        let width = self.width;
        let height = self.height;
        let noise_map = &self.noise_map;

        self.gl.draw(args.viewport(), |context, gl| {
            clear(WHITE, gl);

            let buffer = ImageBuffer::from_fn(width, height, |x, y| {
                let value = (255.0 * noise_map[(x, y)]) as u8;

                Rgba([value, value, value, 255])
            });

            let texture = Texture::from_image(&buffer, &TextureSettings::new());

            image(&texture, context.transform, gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}

    pub fn handle(&mut self, _args: &ButtonArgs) {}
}
