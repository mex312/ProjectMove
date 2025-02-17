use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Main {
    base: Base<Node2D>
}

#[godot_api]
impl Main {
    #[func]
    fn sqrt(&mut self, x: real) -> real {
        real::sqrt(x)
    }
}

#[godot_api]
impl INode2D for Main {
    fn init(base: Base<Node2D>) -> Self {
        Main {
            base
        }
    }
}