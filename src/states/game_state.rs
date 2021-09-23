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
    config::{DEFAULT_GRID_SIZE, CELL_SIZE},
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

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let map_size = (
        DEFAULT_GRID_SIZE * CELL_SIZE,
        DEFAULT_GRID_SIZE * CELL_SIZE,
    );

    let view_size = (
        map_size.0 + 4. * CELL_SIZE,
        map_size.1 + 4. * CELL_SIZE,
    );
    transform.set_translation_xyz(map_size.0 * 0.5, map_size.1 * 0.5, 10.);

    world.create_entity()
        .with(transform)
        .with(Camera::standard_2d(view_size.0, view_size.1))
        .build();
}

fn initialize_player(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let grid = Grid2D::new(0, 0);

    world.create_entity()
        .with(sprite_render)
        .with(Transform::from(grid.clone()))
        .with(grid)
        .with(Player)
        .build();
}

fn initialize_invisible_walls(world: &mut World) {
    let map_size = (
        DEFAULT_GRID_SIZE as i32,
        DEFAULT_GRID_SIZE as i32,
    );
    // top and bottom
    for x in 0..map_size.0 {
        let grid_bottom = Grid2D::new(x, -1);

        world.create_entity()
            .with(Transform::from(grid_bottom.clone()))
            .with(grid_bottom)
            .with(WallInvisible)
            .with(Obstacle)
            .build();

        let grid_top = Grid2D::new(x, map_size.1);

        world.create_entity()
            .with(Transform::from(grid_top.clone()))
            .with(grid_top)
            .with(WallInvisible)
            .with(Obstacle)
            .build();
    }

    for y in 0..map_size.1 {
        let grid_left = Grid2D::new(-1, y);

        world.create_entity()
            .with(Transform::from(grid_left.clone()))
            .with(grid_left)
            .with(WallInvisible)
            .with(Obstacle)
            .build();

        let grid_right = Grid2D::new(map_size.0, y);

        world.create_entity()
            .with(Transform::from(grid_right.clone()))
            .with(grid_right)
            .with(WallInvisible)
            .with(Obstacle)
            .build();
    }
}

fn initialize_floors(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 15);
    let map_size = (
        DEFAULT_GRID_SIZE as i32,
        DEFAULT_GRID_SIZE as i32,
    );
    // top and bottom
    for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            let mut transform: Transform = Grid2D::new(x, y).into();
            transform.set_translation_z(-10.);

            world.create_entity()
                .with(sprite_render.clone())
                .with(transform)
                .build();
        }
    }
}

fn initialize_walls(world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
    let walls = vec![
        (1, 1),
        (3, 5),
        (2, 8),
        (6, 1),
        (11, 11),
        (7, 15)
    ];
    // top and bottom
    for (x, y) in walls {
        let grid = Grid2D::new(x, y);

        world.create_entity()
            .with(sprite_render.clone())
            .with(Transform::from(grid.clone()))
            .with(grid)
            .with(Wall)
            .with(Obstacle)
            .build();
    }
}


fn initialize_goal(world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 2);

    let map_size = (
        DEFAULT_GRID_SIZE as i32,
        DEFAULT_GRID_SIZE as i32,
    );
    let grid = Grid2D::new(map_size.0 - 1, map_size.1 - 1);

    world.create_entity()
        .with(sprite_render.clone())
        .with(Transform::from(grid.clone()))
        .with(grid)
        .with(Goal)
        .build();
}

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Wall>();
        world.register::<WallInvisible>();

        initialize_camera(world);
        initialize_floors(world, sprite_sheet_handle.clone());
        initialize_player(world, sprite_sheet_handle.clone());
        initialize_walls(world, sprite_sheet_handle.clone());
        initialize_goal(world, sprite_sheet_handle);
        initialize_invisible_walls(world);
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