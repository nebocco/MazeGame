use amethyst::{
    prelude::*,
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    renderer::{
        Camera, ImageFormat, SpriteRender,
        SpriteSheet, SpriteSheetFormat, Texture,
        // palette::Srgba,
        // resources::Tint,
    },
};

use crate::{
    components::{
        basics::{Goal, Obstacle, Player, Wall, WallInvisible},
        grid2d::Grid2D,
    },
    resources::{CurrentStageData, GameState},
    config::{DEFAULT_GRID_SIZE, CELL_SIZE},
    states::ClearState,
};

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn create_camera(world: &mut World, map_size: (i32, i32)) {
    let mut transform = Transform::default();
    let map_size = (map_size.0 as f32, map_size.1 as f32);

    let view_size = (map_size.0.max(map_size.1).max(DEFAULT_GRID_SIZE) + 4.) * CELL_SIZE;
    transform.set_translation_xyz(map_size.1 * CELL_SIZE * 0.5, map_size.0 * CELL_SIZE * 0.5, 10.);

    world.create_entity()
        .with(transform)
        .with(Camera::standard_2d(view_size, view_size))
        .build();
}

fn create_player(
    world: &mut World,
    (y, x): (i32, i32),
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let grid = Grid2D::new(x, y);

    world.create_entity()
        .with(sprite_render)
        .with(grid.to_transform(2.))
        .with(grid)
        .with(Player)
        .build();
}

fn create_wall(
    world: &mut World,
    (y, x): (i32, i32),
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
    let grid = Grid2D::new(x, y);

    world.create_entity()
        .with(sprite_render)
        .with(grid.to_transform(1.))
        .with(grid)
        .with(Wall)
        .with(Obstacle)
        .build();
}


fn create_goal(
    world: &mut World,
    (y, x): (i32, i32),
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 2);
    let grid = Grid2D::new(x, y);

    world.create_entity()
        .with(sprite_render.clone())
        .with(grid.to_transform(1.))
        .with(grid)
        .with(Goal)
        .build();
}

fn create_background(
    world: &mut World,
    (height, width): (i32, i32),
    sprite_sheet_handle: Handle<SpriteSheet>)
{
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 15);
    for x in 0..width {
        for y in 0..height {
            let mut transform: Transform = Grid2D::new(x, y).into();
            transform.set_translation_z(-10.);

            world.create_entity()
                .with(sprite_render.clone())
                .with(transform)
                .build();
        }
    }

    // top and bottom
    for x in 0..width {
        let grid_bottom = Grid2D::new(x, -1);

        world.create_entity()
            .with(Transform::from(grid_bottom.clone()))
            .with(grid_bottom)
            .with(WallInvisible)
            .with(Obstacle)
            .build();

        let grid_top = Grid2D::new(x, height);

        world.create_entity()
            .with(Transform::from(grid_top.clone()))
            .with(grid_top)
            .with(WallInvisible)
            .with(Obstacle)
            .build();
    }

    for y in 0..height {
        let grid_left = Grid2D::new(-1, y);

        world.create_entity()
            .with(Transform::from(grid_left.clone()))
            .with(grid_left)
            .with(WallInvisible)
            .with(Obstacle)
            .build();

        let grid_right = Grid2D::new(width, y);

        world.create_entity()
            .with(Transform::from(grid_right.clone()))
            .with(grid_right)
            .with(WallInvisible)
            .with(Obstacle)
            .build();
    }
}

fn load_map(world: &mut World) -> Vec<Vec<char>> {
    let current_stage_data = world.try_fetch::<CurrentStageData>().unwrap();
    let stage = current_stage_data.stage;

    let map = std::fs::read_to_string(format!("./resources/stages/{:02}.txt", stage))
        .unwrap_or("@............\n.###.#.#.##..\n.#...#.#.#.#.\n.###.#.#.#.#.\n.#...#.#.#.#.\n.###.###.##..\n............G".to_string())
        .trim()
        .lines()
        .map(|s| s.chars().collect())
        .collect();
    map
}

fn prepare_stage(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let stage_map = load_map(world);
    let height = stage_map.len() as i32;
    let width = stage_map[0].len() as i32;

    create_camera(world, (height, width));
    create_background(world, (height, width), sprite_sheet_handle.clone());
    for (i, row) in stage_map.iter().enumerate() {
        let i = i as i32;
        for (j, &c) in row.iter().enumerate() {
            let j = j as i32;
            match c {
                '#' => create_wall(world, (i, j), sprite_sheet_handle.clone()),
                '@' => create_player(world, (i, j), sprite_sheet_handle.clone()),
                'G' => create_goal(world, (i, j), sprite_sheet_handle.clone()),
                '.' => (),
                _ => unreachable!(),
            };
        }
    }
}

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
      
        world.delete_all();
        let sprite_sheet_handle = load_sprite_sheet(world);
        prepare_stage(world, sprite_sheet_handle);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(stage_data) = data.world.try_fetch::<CurrentStageData>() {
            if stage_data.state == GameState::Win {
                return Trans::Switch(Box::new(ClearState));
            }
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans {
        if let StateEvent::Window(ref event) = event {
            if is_close_requested(event) || is_key_down(event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }
        Trans::None
    }
}