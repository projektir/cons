use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Core {}

impl Core {
    pub fn new() -> Core {
        Core {}
    }
}

impl Component for Core {
    type Storage = DenseVecStorage<Self>;
}
