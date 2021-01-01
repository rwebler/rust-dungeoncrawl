use crate::prelude::*;
use super::MapArchitect;
pub struct RoomsArchitect {}
impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator, level: Level) -> MapBuilder {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            pike_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant(mb.player_start);
        mb.pike_start = mb.rooms[rng.range(1, super::NUM_ROOMS - 1)].center();
        for room in mb.rooms.iter().skip(1) {
            if rng.range(-1, 3) <= level.level as i32 {
                mb.monster_spawns.push(room.center());
            }
        }
        mb
    }
}