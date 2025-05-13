use godot::classes::{Area2D, Engine, IArea2D, Timer};
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

        body.get_node_or_null("CollisionShape2D")
            .expect("Player should have a node")
            .queue_free();

        Engine::singleton().set_time_scale(0.5);
        self.timer.start();
    }
}

#[godot_api]
impl IArea2D for Killzone {
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
