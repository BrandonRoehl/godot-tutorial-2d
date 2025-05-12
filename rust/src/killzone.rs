use godot::classes::{Area2D, IArea2D, Timer};
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
        self.base().get_tree().unwrap().reload_current_scene();
        // .create_timer(2.0).unwrap();
        // get_tree().unwrap().reload_current_scene().expect("Failed to reload scene");
    }

    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("You have died");

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
