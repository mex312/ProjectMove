use godot::prelude::*;
use godot::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Unit {
    #[var]
    speed: f64,
    base: Base<Node2D>
}

#[godot_api]
impl Unit {

}

#[godot_api]
impl INode2D for Unit {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            speed: 1.,
            base
        }
    }

    fn draw(&mut self) {
        self.base_mut().draw_circle(Vector2::new(0., 0.), 10., Color::RED);
    }
}