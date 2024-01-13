// This is just following the Godot-rust tutorial for a player class. We should probably change this just a bit.

use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    hp: i64,
    speed: f64,
    angular_speed: f64,

    #[base]
    base: Base<Sprite2D>
}