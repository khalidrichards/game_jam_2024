
use godot::engine::{Area2D, IArea2D, PhysicsBody2D, CollisionShape2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    hp: i64,
    strength: i16,
    agility: i16,
    toughness: i16,
    knowledge: i16,
    will: i16,
    luck: i16,
    speed: real,
    screen_size: Vector2,


    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Player {

    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }

}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            hp: 100,
            strength: 0,
            agility: 0,
            toughness: 0,
            knowledge: 0,
            will: 0,
            luck: 0,
            speed: 120.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut();
    }

    fn process(&mut self, delta: f64) {
        let sprite = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        
        let input = Input::singleton();
        if input.is_action_pressed("right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("up".into()) {
            velocity += Vector2::UP;
        }


        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}