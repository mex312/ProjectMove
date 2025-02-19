use godot::classes::{ITileMapLayer, TileMapLayer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct World {
    base: Base<TileMapLayer>
}

#[godot_api]
impl World {
    #[func]
    fn sqrt(&mut self, x: real) -> real {
        real::sqrt(x)
    }
}

#[godot_api]
impl ITileMapLayer for World {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base
        }
    }
}