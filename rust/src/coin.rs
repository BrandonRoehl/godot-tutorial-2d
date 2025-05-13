use godot::classes::{Area2D, IArea2D};
use godot::global::*;
use godot::prelude::*;

use crate::state;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Coin {
    base: Base<Area2D>,

    #[init(val = OnReady::from_node("%State"))]
    state: OnReady<Gd<state::State>>,
}

#[godot_api]
impl Coin {
    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("Coin collected by player");
        // let mut state = self
        //     .base()
        //     .try_get_node_as::<state::State>("%State")
        //     .expect("State node must exist");
        self.state.bind_mut().add_score(1);

        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for Coin {
    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
    }
}
