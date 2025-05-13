use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::global::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
struct Player {
    #[export]
    #[init(val = 130.0)]
    speed: real,

    #[export]
    #[init(val = -300.0)]
    jump_velocity: real,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    // On frame rates higher than 60 FPS, this function is not called enough so
    // we see jitters appear in gameplay. This should be converted to have player
    // movement happen in `process` instead of `physics_process`.
    // This should still be called for collision detection but the horizontal
    // movement should be removed from this function call.
    //
    // `delta` can be f32 or f64; #[godot_api] macro converts transparently.
    fn physics_process(&mut self, delta: f32) {
        // godot_print!("processing {delta}"); // Prints to the Godot console

        // This is the mut base we are going to borrow mutate and release
        // get the speed and the input
        let jump_velocity = self.jump_velocity;
        let speed = self.speed;
        let mut base = self.base_mut();
        let mut velocity = base.get_velocity();
        let input = Input::singleton();

        // Vertical movement
        if !base.is_on_floor() {
            // Gravity
            velocity += base.get_gravity() * delta;
        } else if input.is_action_just_pressed("ui_accept") {
            // Jump
            velocity.y = jump_velocity;
        }

        // Horizontal movement
        // Get the input direction and handle the movement/deceleration.
        // As good practice, you should replace UI actions with custom gameplay actions.
        let direction = input.get_axis("ui_left", "ui_right");
        if direction != 0.0 {
            velocity.x = direction * speed;
        } else {
            // Deceleration
            velocity.x = move_toward(velocity.x as f64, 0.0, speed as f64) as f32;
        }

        base.set_velocity(velocity);
        base.move_and_slide();
    }
}
