use azure::azure_hl::{DrawTarget};
use geom::rect::Rect;

pub struct LayerBuffer {
    draw_target: DrawTarget,

    // The rect in the containing RenderLayer that this represents.
    rect: Rect<uint>,

    // NB: stride is in pixels, like OpenGL GL_UNPACK_ROW_LENGTH.
    stride: uint
}

/// A set of layer buffers. This is an atomic unit used to switch between the front and back
/// buffers.
pub struct LayerBufferSet {
    buffers: ~[LayerBuffer]
}

/**
The interface used to by the renderer to aquire draw targets for
each rendered frame and submit them to be drawn to the display
*/
pub trait Compositor {
    fn begin_drawing(&self, next_dt: comm::Chan<LayerBufferSet>);
    fn draw(&self, next_dt: comm::Chan<LayerBufferSet>, +draw_me: LayerBufferSet);
}

