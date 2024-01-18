
use godot::engine::{Area2D, IArea2D, PhysicsBody2D, CollisionShape2D, Sprite2D};
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
        let pos = Vector2::ZERO;
        
    }

    fn process(&mut self, delta: f64) {
        
          
     }
      
       
    

}