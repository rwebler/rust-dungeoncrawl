use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x ..= camera.right_x {
            let pt = Point::new(x, y);
            let ptt = Point::new(x, y-1);
            let ptb = Point::new(x, y+1);
            let ptl = Point::new(x-1, y);
            let ptr = Point::new(x+1, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt)
                    || (player_fov.visible_tiles.contains(&ptt)
                            && player_fov.visible_tiles.contains(&ptb)
                        )
                    || (player_fov.visible_tiles.contains(&ptl)
                            && player_fov.visible_tiles.contains(&ptr)
                        )
            ) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(
                        RGB::named(WHITE),
                        RGB::named(BLACK),
                    ),
                    glyph
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}