use godot::prelude::*;
mod coin;
mod player;

struct TutorialBase;

#[gdextension]
unsafe impl ExtensionLibrary for TutorialBase {}
