use godot::prelude::*;
mod coin;
mod killzone;
mod player;

struct TutorialBase;

#[gdextension]
unsafe impl ExtensionLibrary for TutorialBase {}
