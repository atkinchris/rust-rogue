use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
  Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
  Texture, TextureMetadata,
};

pub struct Gameplay;

pub const TILE_SIZE: f32 = 16.0;
pub const CAMERA_WIDTH: f32 = 50.0 * TILE_SIZE;
pub const CAMERA_HEIGHT: f32 = 40.0 * TILE_SIZE;

fn initialise_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_z(1.0);
  world
    .create_entity()
    .with(Camera::from(Projection::orthographic(
      0.0,
      CAMERA_WIDTH,
      0.0,
      CAMERA_HEIGHT,
    )))
    .with(transform)
    .build();
}

fn initialise_entities(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
  let mut transform = Transform::default();
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle.clone(),
    sprite_number: 0,
  };

  transform.set_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.0);

  world
    .create_entity()
    .with(transform)
    .with(sprite_render)
    .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "sprites.png",
      PngFormat,
      TextureMetadata::srgb_scale(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "sprites.ron",
    SpriteSheetFormat,
    texture_handle,
    (),
    &sprite_sheet_store,
  )
}

impl SimpleState for Gameplay {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    let sprite_sheet_handle = load_sprite_sheet(world);

    initialise_entities(world, sprite_sheet_handle.clone());
    initialise_camera(world);
  }
}
