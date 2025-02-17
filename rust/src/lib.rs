use godot::prelude::*;

mod main_scene;
mod unit;

struct ProjectMove;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectMove {}