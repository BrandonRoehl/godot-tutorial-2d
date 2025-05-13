use godot::classes::{AnimatedSprite2D, INode2D, Node2D, RayCast2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Slime {
    #[export(range = (0.0, 100.0, or_greater))]
    #[init(val = 60.0)]
    speed: real,

    // These can take values enums and more
    #[export(enum = (Left, Right))]
    #[init(val = Direction::Right)]
    direction: Direction,

    #[init(val = OnReady::from_node("RayCastRight"))]
    ray_right: OnReady<Gd<RayCast2D>>,

    #[init(val = OnReady::from_node("RayCastLeft"))]
    ray_left: OnReady<Gd<RayCast2D>>,

    #[init(val = OnReady::from_node("AnimatedSprite2D"))]
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,

    base: Base<Node2D>,
}

#[godot_api]
impl Slime {
    /// Raycast and invert direction if necessary
    fn raycast(&mut self) {
        let (ray, result) = match self.direction {
            Direction::Right => (&self.ray_right, Direction::Left),
            Direction::Left => (&self.ray_left, Direction::Right),
        };
        if ray.is_colliding() {
            self.direction = result;
            result.set_flip_h(&mut self.animated_sprite);
            godot_print!("New direction {result}");
        }
    }
}

#[godot_api]
impl INode2D for Slime {
    fn process(&mut self, delta: f32) {
        self.raycast();

        let speed = self.direction.as_real() * self.speed;

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
    Right,
    Left,
}

impl Direction {
    fn as_real(&self) -> real {
        match self {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
        }
    }

    fn set_flip_h(&self, sprite: &mut Gd<AnimatedSprite2D>) {
        match self {
            Direction::Right => sprite.set_flip_h(false),
            Direction::Left => sprite.set_flip_h(true),
        }
    }

    /// Invert the direction in place
    fn _invert(&mut self) {
        *self = match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
        }
    }
}
