use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::systems::PlayerMoveSystem;

mod components;
mod config;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let display_config = resources.join("display.ron");

    // let input_bundle = InputBundle::<StringBindings>::new()
    //     .with_bindings_from_file(resources.join("input.ron"))?;

    let game_data = GameDataBuilder::new()
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(PlayerMoveSystem::default(), "player_move_system", &["input_system"]);

    let mut game = Application::new(resources, states::GameState, game_data)?;
    game.run();

    Ok(())
}
