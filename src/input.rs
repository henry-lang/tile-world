use glam::Vec2;
use glium::glutin::event::{
    ElementState::{Pressed, Released},
    Event::{self, WindowEvent},
    KeyboardInput, VirtualKeyCode,
    WindowEvent::{CursorMoved, KeyboardInput as WindowKeyboardInput},
};
use rbitset::BitSet256;

#[derive(Default)]
pub struct Input {
    cursor_pos: Vec2,
    keys_down: BitSet256, // Currently hardcoding max supported virtual keycodes. I doubt it's an issue though.
    keys_pressed: BitSet256,
    keys_released: BitSet256,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, event: &Event<()>) {
        self.keys_pressed.clear();
        self.keys_released.clear();

        match event {
            WindowEvent { event, .. } => match event {
                WindowKeyboardInput {
                    input:
                        KeyboardInput {
                            state: Pressed,
                            virtual_keycode: Some(vkey),
                            ..
                        },
                    ..
                } => {
                    self.keys_pressed.insert(*vkey as usize);
                    self.keys_down.insert(*vkey as usize);
                }

                WindowKeyboardInput {
                    input:
                        KeyboardInput {
                            state: Released,
                            virtual_keycode: Some(vkey),
                            ..
                        },
                    ..
                } => {
                    self.keys_down.remove(*vkey as usize);
                    self.keys_released.insert(*vkey as usize);
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }

    pub fn is_key_down(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_down.contains(keycode as usize)
    }

    pub fn is_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(keycode as usize)
    }

    pub fn is_key_released(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_released.contains(keycode as usize)
    }
}
