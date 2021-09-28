use amethyst::{
    prelude::*,
    core::transform::Transform,
    assets::{Handle, Loader},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    renderer::Camera,
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform},
};

use crate::{
    components::basics::{Wall, WallInvisible},
    config::{DEFAULT_GRID_SIZE, CELL_SIZE},
    resources::CurrentStageData,
    states::PlayState,
};

fn set_resources(world: &mut World) {
    world.insert(CurrentStageData::default());
}

fn create_start_window(world: &mut World) {
    let window_size = DEFAULT_GRID_SIZE * CELL_SIZE;

    let font_handle: Handle<FontAsset> = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    
    // create camera
    {
        let mut transform = Transform::default();
        transform.set_translation_xyz(window_size * 0.5, window_size * 0.5, 10.);

        world.create_entity()
            .with(transform)
            .with(Camera::standard_2d(window_size, window_size))
            .build();
    }

    // create title
    {
        let transform = UiTransform::new(
            "title".to_string(), Anchor::Middle, Anchor::Middle,
            0., 20., 9.5, 500., 200.,
        );

        let text = UiText::new(
            font_handle.clone(),
            "title".to_string(),
            [1., 1., 1., 1.],
            60.,
            LineMode::Single,
            Anchor::Middle,
        );

        world
            .create_entity()
            .with(transform)
            .with(text)
            .build();
    }

    {
        let transform = UiTransform::new(
            "press enter".to_string(), Anchor::Middle, Anchor::Middle,
            0., -40., 9.5, 500., 200.,
        );

        let text = UiText::new(
            font_handle.clone(),
            "press enter".to_string(),
            [1., 1., 1., 1.],
            36.,
            LineMode::Single,
            Anchor::Middle,
        );

        world
            .create_entity()
            .with(transform)
            .with(text)
            .build();
    }
}

pub struct LoadState;

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        set_resources(world);
        
        world.delete_all();

        world.register::<Wall>();
        world.register::<WallInvisible>();

        create_start_window(world)
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans {
        if let StateEvent::Window(ref event) = event {
            if is_close_requested(event) || is_key_down(event, VirtualKeyCode::Escape) {
                return Trans::Quit
            } else if is_key_down(event, VirtualKeyCode::Return) {
                return Trans::Switch(Box::new(PlayState));
            }
        }
        Trans::None
    }
}