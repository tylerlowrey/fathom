use bevy::prelude::Event;
use winit::event::KeyEvent;

#[derive(Event)]
pub enum InputEvent {
    Keyboard(KeyEvent)
}