use amethyst::{
    prelude::*,
    assets::{Handle, Loader},
    // ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{is_key_down, VirtualKeyCode},
    renderer::{
        palette::{Srgba, Pixel},
    },
    ui::{
        UiImage, Anchor, UiTransform, TtfFormat, FontAsset, UiText, LineMode
    }
};

use crate::{
    config::{DEFAULT_GRID_SIZE, CELL_SIZE},
    states::PlayState,
};

pub struct ClearState;

impl SimpleState for ClearState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        let map_size = (
            DEFAULT_GRID_SIZE * CELL_SIZE,
            DEFAULT_GRID_SIZE * CELL_SIZE,
        );
    
        let _view_size = (
            map_size.0 + 4. * CELL_SIZE,
            map_size.1 + 4. * CELL_SIZE,
        );

        let color = UiImage::SolidColor(
            Srgba::new(0.05, 0.02, 0.06, 0.8)
                .into_linear()
                .into_raw()
        );

        let transform = UiTransform::new(
            "screen".to_string(), Anchor::Middle, Anchor::Middle,
            0., 0., 9., 550., 550.,
        );

        world.create_entity()
            // .with(e)
            .with(transform)
            .with(color)
            .build();

        let font_handle: Handle<FontAsset> = world.read_resource::<Loader>().load(
            "fonts/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(UiTransform::new(
                "P2".to_string(), Anchor::Middle, Anchor::Middle,
                0., 20., 9.5, 500., 200.,
            ))
            .with(UiText::new(
                font_handle.clone(),
                "Stage Clear".to_string(),
                [1., 1., 1., 1.],
                40.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .build();

            world
            .create_entity()
            .with(UiTransform::new(
                "P2".to_string(), Anchor::Middle, Anchor::Middle,
                0., -20., 9.5, 500., 200.,
            ))
            .with(UiText::new(
                font_handle.clone(),
                "press enter".to_string(),
                [1., 1., 1., 1.],
                20.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .build();

    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(ref event) = event {
            if is_key_down(event, VirtualKeyCode::Return) {
                return Trans::Switch(Box::new(PlayState));
            }
        }
        Trans::None
    }
}