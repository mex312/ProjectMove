use godot::prelude::*;
use crate::world::World;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Unit {
    #[export]
    #[var]
    world: Option<Gd<World>>,
    base: Base<Node2D>
}

#[godot_api]
impl Unit {

}

#[godot_api]
impl INode2D for Unit {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            world: None,
            base
        }
    }

    fn draw(&mut self) {
        self.base_mut().draw_circle(Vector2::new(0., 0.), 8., Color::RED);
    }

    fn ready(&mut self) {
        //self.world = Option::from(self.base().get_parent().expect("No parent node").try_cast::<World>().expect("Parent node is not World"));

        if self.world.is_none() {godot_error!("No world attached!"); return}

        let new_pos =
            self.world.as_ref().unwrap().map_to_local(
                self.world.as_ref().unwrap().local_to_map(
                    self.base().get_position()
                )
            );

        self.base_mut().set_position(new_pos - Vector2::new(0.5, 0.5));
    }
}