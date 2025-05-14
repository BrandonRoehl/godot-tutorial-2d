use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input};
use godot::global::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
struct Player {
    // @Export is the Godot equivalent of #[export] this is what makes it
    // available in the editor.
    #[export(range = (0.0, 100.0, or_greater))]
    #[init(val = 130.0)]
    speed: real,

    // #[export] // these can have validations on them
    #[export(range = (-100.0, 0.0, or_less))]
    #[init(val = -300.0)]
    jump_velocity: real,

    #[init(val = OnReady::from_node("AnimatedSprite2D"))]
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    /// On frame rates higher than 60 FPS, this function is not called enough so
    /// we see jitters appear in gameplay. This should be converted to have player
    /// movement happen in `process` instead of `physics_process`.
    /// This should still be called for collision detection but the horizontal
    /// movement should be removed from this function call.
    ///
    /// `delta` can be f32 or f64; #[godot_api] macro converts transparently.
    fn physics_process(&mut self, delta: f32) {
        // godot_print!("processing {delta}"); // Prints to the Godot console

        // Mutate the base node
        let mut velocity: Vector2 = self.base().get_velocity();
        let input = Input::singleton();

        // Vertical movement
        if !self.base().is_on_floor() {
            // Gravity
            velocity += self.base().get_gravity() * delta;
        } else if input.is_action_just_pressed("jump") {
            // Jump
            velocity.y = self.jump_velocity;
        }

        // Horizontal movement
        // Get the input direction and handle the movement/deceleration.
        // As good practice, you should replace UI actions with custom gameplay actions.
        // Float range (-1.0..=1.0)
        let direction = input.get_axis("move_left", "move_right");
        if direction == 0.0 {
            // Deceleration instead of snapping to 0
            velocity.x = move_toward(velocity.x as f64, 0.0, self.speed as f64) as f32;
        } else {
            velocity.x = direction * self.speed;
            self.animated_sprite.set_flip_h(direction < 0.0);
        }

        // Set the animation based on the state of the player
        let animation = match (direction, self.base().is_on_floor()) {
            (_, false) => "jump",
            (0.0, true) => "idle",
            (_, true) => "run",
        };
        self.animated_sprite.play_ex().name(animation).done();

        // At the end mutably borrow the base to set what we need
        let mut base = self.base_mut();
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}
