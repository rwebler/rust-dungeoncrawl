use crate::prelude::*;

use super::MapArchitect;
pub struct ArenaArchitect {}
impl MapArchitect for ArenaArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            potion_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            pike_start: Point::zero(),
        };
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let idx = map_idx(x, y);
                if x == 1 || y == 1 || x == SCREEN_WIDTH-1 || y == SCREEN_HEIGHT-1 {
                    mb.map.tiles[idx] = TileType::Wall;
                } else {
                    mb.map.tiles[idx] = TileType::Floor;
                }
            }
        }
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant(mb.player_start);
        mb.pike_start = mb.find_most_distant(mb.amulet_start);
        for _ in 0..50 {
            mb.monster_spawns.push(
                Point::new(
                    rng.range(2, SCREEN_WIDTH - 2),
                    rng.range(2, SCREEN_HEIGHT - 2)
                )
            );
        };
        mb
    }
}