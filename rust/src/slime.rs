use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Slime {
    #[export]
    #[init(val = 60.0)]
    speed: real,

    #[export]
    #[init(val = Direction::East)]
    direction: Direction,

    base: Base<Area2D>,
}

// #[godot_api]
// impl Slime {
//     fn on_body_entered(&mut self, _body: Gd<Node2D>) {
//         godot_print!("You have died");
//     }
// }

#[godot_api]
impl IArea2D for Slime {
    fn process(&mut self, delta: f32) {
        let speed = self.direction.to_f32() * self.speed;

        let mut base = self.base_mut();
        let mut pos = base.get_position();

        pos.x += delta * speed;

        base.set_position(pos);
    }
}

/// A direction enum.
#[derive(GodotConvert, Var, Export, Copy, Clone, Debug)]
#[godot(via = GString)]
enum Direction {
    East,
    West,
}

impl Direction {
    fn to_f32(&self) -> f32 {
        match self {
            Direction::East => 1.0,
            Direction::West => -1.0,
        }
    }
}
