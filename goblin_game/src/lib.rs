use godot::engine::ISprite2D;
use godot::engine::Sprite2D;
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    hp: i64,
    speed: f64,
    angular_speed: f64,

    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello world!"); // godot console print

        Self {
            hp: 100,
<<<<<<< HEAD
<<<<<<< HEAD
            speed: 1000.0,
=======
            speed: 400.0,
>>>>>>> d9ec382 (Wrapping up the base project. We can iterate and use this to build our project)
=======
            speed: 1000.0,
>>>>>>> 9fcd837 (Adding first scene stuff)
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}
