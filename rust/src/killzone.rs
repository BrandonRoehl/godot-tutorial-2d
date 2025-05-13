use godot::classes::{Area2D, CollisionShape2D, Engine, IArea2D, Timer};
use godot::global::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Killzone {
    base: Base<Area2D>,
    timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl Killzone {
    fn on_timer_timeout(&mut self) {
        Engine::singleton().set_time_scale(1.0);

        self.base()
            .get_tree()
            .expect("Node must exist in a scene")
            .reload_current_scene();
    }

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        godot_print!("You have died");

        if !body.is_class("Player") {
            // Soft error here
            godot_error!("Body is not a player");
            return;
        }

        let mut collision_shape = body
            .try_get_node_as::<CollisionShape2D>("CollisionShape2D")
            .expect("Player should have a node");

        // Following is the one from the tutorial but I prefer this approach
        collision_shape.set_deferred("disabled", &true.to_variant());
        // collision_shape.queue_free();

        Engine::singleton().set_time_scale(0.5);
        self.timer.start();
    }
}

#[godot_api]
impl IArea2D for Killzone {
    /// Leaving this init to showcase an alternative way to initialize if you
    /// need to do something more than just a standard value assignment.
    fn init(base: Base<Area2D>) -> Self {
        Self {
            timer: OnReady::from_node("Timer"),
            base,
        }
    }

    fn ready(&mut self) {
        let zone = self.to_gd();

        godot_print!("Zone ready");

        self.signals()
            .body_entered()
            .connect_obj(&zone, Self::on_body_entered);

        self.timer
            .signals()
            .timeout()
            .connect_obj(&zone, Self::on_timer_timeout);
    }
}
