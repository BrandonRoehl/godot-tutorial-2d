use godot::classes::{Area2D, IArea2D};
use godot::global::*;
use godot::prelude::*;

use crate::state::State;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl Coin {
    #[signal]
    fn collected(ammount: i64);

    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        godot_print!("Coin collected by player");

        // Emit the signal 1. You can call the method directly but hooking
        // signals together is good practice as it allows a base state to
        // observe more where these are just hooked up on creation
        self.signals().collected().emit(1);
        // To call it directly reference the `bind_mut` either on a `OnReady`
        // such as
        // ```rust
        // self.state.bind_mut().add_score(1);
        // ```
        // or via
        // ```rust
        // let state = self
        //     .base()
        //     .try_get_node_as::<State>("%State")
        //     .expect("State node must exist")
        //     .bind_mut()
        //     .add_score(1);
        // ```
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for Coin {
    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);

        let state = self
            .base()
            .try_get_node_as::<State>("%State")
            .expect("State node must exist");

        // Hook up the signal no need to save the state here or a reference
        self.signals()
            .collected()
            .connect_obj(&state, State::add_score)
    }
}
