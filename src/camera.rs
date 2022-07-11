use clamp::clamp;
use glam::{Mat4, Vec2, Vec3};
use glium::{glutin::event::VirtualKeyCode, Display};

use std::ops::Range;
use std::rc::Rc;

use crate::Input;

// struct Renderer {
//     camera: Camera,
// }

pub struct Camera {
    display: Rc<Display>,
    position: Vec2,
    zoom: f32,
    zoom_range: Range<f32>,
}

impl Camera {
    pub fn new(display: Rc<Display>, position: Vec2, zoom: f32, zoom_range: Range<f32>) -> Self {
        Self {
            display,
            position,
            zoom: clamp(zoom_range.start, zoom, zoom_range.end),
            zoom_range,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let speed = 10. * (1. / self.zoom);

        if input.is_key_down(VirtualKeyCode::W) {
            self.position.y -= speed;
        }
        if input.is_key_down(VirtualKeyCode::S) {
            self.position.y += speed;
        }

        if input.is_key_down(VirtualKeyCode::A) {
            self.position.x -= speed;
        }

        if input.is_key_down(VirtualKeyCode::D) {
            self.position.x += speed;
        }

        if input.is_key_down(VirtualKeyCode::Q) {
            self.zoom = clamp(self.zoom_range.start, self.zoom - 0.5, self.zoom_range.end);
        }

        if input.is_key_down(VirtualKeyCode::E) {
            self.zoom = clamp(self.zoom_range.start, self.zoom + 0.5, self.zoom_range.end);
        }
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let (width, height) = (width as f32, height as f32);

        let left = self.position.x - width * 0.5;
        let right = self.position.x + width * 0.5;
        let top = self.position.y - height * 0.5;
        let bottom = self.position.y + height * 0.5;

        let ortho = Mat4::orthographic_lh(left, right, bottom, top, 0.01, 100.);
        let zoom = Mat4::from_scale(Vec3::splat(self.zoom));

        ortho * zoom
    }
}
