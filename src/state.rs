use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    window::ScreenDimensions,
    renderer::{Camera, ImageFormat, SpriteSheetFormat, SpriteSheet, SpriteRender, Texture},
};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_camera(world, &dimensions);

        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/core.gif",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/core.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    vec!(SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    })
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    for (i, sprite) in sprites.iter().enumerate() {
        let x = (i as f32 - 1.) * 32. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 32. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }
}
