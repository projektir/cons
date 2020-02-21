use amethyst::{
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    ecs::{Read, System, SystemData},
};

#[derive(SystemDesc)]
pub struct MainSystem {
    pub pressed: bool,
}

impl<'s> System<'s> for MainSystem {
    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {
        if let Some((x, y)) = input.mouse_position() {
            let mouse_down = input.action_is_down("click").unwrap_or(false);

            if self.pressed && !mouse_down {
                log::info!("x: {}, y: {}, clicked!", x, y);
                self.pressed = false;
            } else if mouse_down {
                self.pressed = true;
            }
        }
    }
}
