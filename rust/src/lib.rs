use godot::prelude::*;
mod player;

struct TutorialBase;

#[gdextension]
unsafe impl ExtensionLibrary for TutorialBase {}
