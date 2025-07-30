use bevy::prelude::*;
// use prelude::*;
use bevy_ecs_tilemap::prelude::*;


// used as an example for now

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size= TilemapSize { x:32,y:32};

    let tilemap_entity= commands.spawn_empty().id();

    let mut tile_storage=TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });

    //  // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // // Only used when the atlas feature is off and we are using array textures.
    // #[cfg(all(not(feature = "atlas"), feature = "render"))]
    // {
    //     array_texture_loader.add(TilemapArrayTexture {
    //         texture: TilemapTexture::Single(asset_server.load("tiles.png")),
    //         tile_size,
    //         ..Default::default()
    //     });
    // }

}