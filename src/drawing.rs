use crate::ecs::components::Image;
use ggez::graphics::{Drawable, DrawParam, Rect, DrawMode, Mesh, WHITE, BlendMode, draw};
use ggez::{Context, GameResult};
use crate::constants::{IMAGE_SCALE_FACTOR, SCREEN_WIDTH, SCREEN_HEIGHT};
use ncollide2d::na::{Point2, Vector2};

impl Drawable for Image {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let rect = Rect::new(-0.5, -0.5, 1.0, 1.0);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, WHITE)?;

        &self.vectors.iter()
            .map(|vector| {
                let dest = nalgebra::Point2::from(param.dest);
                dest + vector * IMAGE_SCALE_FACTOR
            })
            .map(|point: nalgebra::Point2<f32>| {
                let flipped_point = Point2::new(point.x, -point.y);
                flipped_point + Vector2::new(SCREEN_WIDTH/2.0, SCREEN_HEIGHT/2.0)
            })
            .map(|dest: nalgebra::Point2<f32>| {
                dest.into()
            })
            .for_each(|dest: mint::Point2<f32>| {
                draw(
                    ctx,
                    &mesh,
                    DrawParam::default()
                        .dest(dest)
                        .scale(mint::Vector2::from([IMAGE_SCALE_FACTOR, IMAGE_SCALE_FACTOR]))
                        .color(self.kind.get_color()),
                ).unwrap_or_else(|err| println!("draw error {:?}", err));
            });
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

pub fn world_to_screen_coordinates(screen_width: f32, screen_height: f32, world_point: mint::Point2<f32>) -> mint::Point2<f32>{
    mint::Point2 {
        x: world_point.x + screen_width/2.0,
        y: -world_point.y + screen_height/2.0
    }
}
