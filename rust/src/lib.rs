use godot::prelude::*;

mod world;
mod unit;

struct ProjectMove;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectMove {}