use godot::prelude::*;
use godot::classes::{ISprite2D, Sprite2D};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct NimBot { 
    time : f64,
    base: Base<Node> 
}

#[godot_api]
impl INode for NimBot {
    fn init(base: Base<Node>) -> Self {
        Self { time : 0.0, base }
    }
    fn physics_process(&mut self, delta : f64) {
        self.time += delta;
        let time = self.time;
        godot_print!("{time:.3}");
    }
}