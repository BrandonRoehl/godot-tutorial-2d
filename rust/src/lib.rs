use godot::prelude::*;
mod coin;
mod killzone;
mod player;
mod slime;
mod state;

struct TutorialBase;

#[gdextension]
unsafe impl ExtensionLibrary for TutorialBase {}
