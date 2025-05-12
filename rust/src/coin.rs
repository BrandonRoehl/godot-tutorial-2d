use godot::classes::{Area2D, IArea2D};
use godot::global::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl Coin {
    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("Coin collected by player");

        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Coin is ready");

        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);

        // .connect_self()
        // .connect(
        //     self.base(),
        //     "on_body_entered",
        //     GodotObject::null(),
        // )
    }
}
