use crate::prelude::*;

mod drunkard;
use drunkard::DrunkardsWalkArchitect;

mod rooms;
use rooms::RoomsArchitect;

mod automata;
use automata::CellularAutomataArchitect;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator, level: Level) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub potion_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub pike_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
    fn find_most_distant(&mut self, target: Point) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(target)],
            &self.map,
            1024.0
        );
        self.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < &std::f32::MAX)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        )
    }
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10)
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate() {
            if i != 0 {
                let prev = rooms[i-1].center();
                let new = room.center();
                if rng.range(0,2) == 1 {
                    self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                    self.apply_vertical_tunnel(prev.y, new.y, new.x);
                } else {
                    self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                    self.apply_horizontal_tunnel(prev.x, new.x, new.y);

                }
            }
        }
    }
    fn spawn_monsters(
        &self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        level: Level
    ) -> Vec<Point> {
        let mut spawnable_tiles: Vec<Point> = self.map.tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)|
                (**t == TileType::Floor || **t == TileType::Ground) &&
                DistanceAlg::Pythagoras.distance2d(
                    *start,
                    self.map.index_to_point2d(*idx)
                ) > 10.0
            )
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..15 * (level.level+1) {
            let target_index = rng.random_slice_index(&spawnable_tiles)
                .unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
    fn spawn_potions(
        &self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        level: Level
    ) -> Vec<Point> {
        let mut spawns = Vec::new();
        if level.level == 0 {
            return spawns;
        }
        let mut spawnable_tiles: Vec<Point> = self.map.tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)|
                (**t == TileType::Floor || **t == TileType::Ground) &&
                DistanceAlg::Pythagoras.distance2d(
                    *start,
                    self.map.index_to_point2d(*idx)
                ) > 15.0
            )
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        for _ in 0..2 * level.level {
            let target_index = rng.random_slice_index(&spawnable_tiles)
                .unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
    pub fn build(rng: &mut RandomNumberGenerator, level: Level) -> Self {
        let mut architect : Box<dyn MapArchitect> = Box::new(RoomsArchitect{});
        if level.level< 3 {
            architect = match level.level {
                0 => Box::new(CellularAutomataArchitect{}),
                1 => Box::new(RoomsArchitect{}),
                _ => Box::new(DrunkardsWalkArchitect{})
            };
        }
        let mb = architect.build(rng, level);
        mb
    }
}