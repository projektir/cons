use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use log::info;

mod states;
mod systems;

mod core;

use crate::systems::MainSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");
    let input_bindings = resources.join("input_bindings.ron");

    info!("Hello, world!");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(input_bindings)?;

    let game_data = GameDataBuilder::default()
        
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.05, 0.05, 0.05, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(MainSystem { pressed: false }, "main_system", &["input_system"]);

    let mut game = Application::new(resources, states::MainState, game_data)?;
    game.run();

    Ok(())
}
