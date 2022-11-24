use super::{position::{TryGetPosition}, sourcemap::RenderContext};

pub trait Render: TryGetPosition {
    fn render(&self, context: &mut RenderContext);
}