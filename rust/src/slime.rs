use godot::classes::{AnimatedSprite2D, INode2D, Node2D, RayCast2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Slime {
    // #[init(val = 60.0)]
    #[export(range = (0.0, 100.0, or_greater))]
    speed: real,

    // these can take values enums and more
    // #[init(val = Direction::Right)]
    #[export(enum = (Left, Right))]
    direction: Direction,

    base: Base<Node2D>,

    // MARK: - links
    rays: (OnReady<Gd<RayCast2D>>, OnReady<Gd<RayCast2D>>),
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,
}

impl Slime {
    /// Raycast and invert direction if necessary
    fn raycast(&mut self) {
        let (ray, result) = match self.direction {
            Direction::Right => (&self.rays.1, Direction::Left),
            Direction::Left => (&self.rays.0, Direction::Right),
        };
        if ray.is_colliding() {
            self.direction = result;
            result.set_flip_h(&mut self.animated_sprite);
        }
    }
}

#[godot_api]
impl INode2D for Slime {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            speed: 60.0,
            direction: Direction::Right,
            rays: (
                OnReady::from_node("RayCastLeft"),
                OnReady::from_node("RayCastRight"),
            ),
            animated_sprite: OnReady::from_node("AnimatedSprite2D"),
            base,
        }
    }

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

    fn set_flip_h(&self, sprite: &mut OnReady<Gd<AnimatedSprite2D>>) {
        match self {
            Direction::Right => sprite.set_flip_h(false),
            Direction::Left => sprite.set_flip_h(true),
        }
    }

    // fn invert(&mut self) {
    //     *self = match self {
    //         Direction::Right => Direction::Left,
    //         Direction::Left => Direction::Right,
    //     }
    // }
}
