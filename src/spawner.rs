pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point, damage: i32, current: i32) {
    ecs.push(
        (
            Player{
                damage,
            },
            pos,
            Render{
                color: ColorPair::new(
                    RGB::named(WHITE),
                    RGB::named(BLACK)
                ),
                glyph: to_cp437('@')
            },
            Health{
                current,
                max: 10,
            },
            FieldOfView::new(8),
        )
    );
}

fn goblin() -> (i32, String, FontCharType, i32) {
    (1, "Goblin".to_string(), to_cp437('g'), 7)
}
fn orc() -> (i32, String, FontCharType, i32) {
    (2, "Orc".to_string(), to_cp437('o'), 5)
}
fn orc_brute() -> (i32, String, FontCharType, i32) {
    (3, "Brute".to_string(), to_cp437('O'), 4)
}
fn ettin() -> (i32, String, FontCharType, i32) {
    (4, "Ettin".to_string(), to_cp437('E'), 3)
}
pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point,
    level: Level
) {
    let (hp, name, glyph, radius) = match rng.roll_dice(1, 12) + (level as i32) {
        1..=7 => goblin(),
        8..=11 => orc(),
        12..=13 => orc_brute(),
        _ => ettin(),
    };
    println!("Spawning {} @ {},{}", name, pos.x, pos.y);
    ecs.push(
        (
            Enemy,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph
            },
            ChasingPlayer{},
            Health{current: hp, max: hp},
            Name(name),
            FieldOfView::new(radius),
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    println!("Spawning amulet @ {},{}", pos.x, pos.y);
    ecs.push(
        (
            Item,
            AmuletOfYala,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|')
            },
            Name("Amulet of Yala".to_string())
        )
    );
}

pub fn spawn_pike_of_destiny(ecs: &mut World, pos: Point) {
    println!("Spawning pike @ {},{}", pos.x, pos.y);
    ecs.push(
        (
            Item,
            PikeOfDestiny,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('\u{2191}')
            },
            Name("Pike of Destiny".to_string())
        )
    );
}

pub fn spawn_entrance(ecs: &mut World, pos: Point) {
    println!("Spawning entrance @ {},{}", pos.x, pos.y);
    ecs.push(
        (
            Item,
            Entrance,
            pos,
            Render {
                color: ColorPair::new(YELLOW, BLACK),
                glyph: to_cp437('>')
            },
            Name("Entrance".to_string())
        )
    );
}

pub fn spawn_potion(
    ecs: &mut World,
    pos: Point,
) {
    println!("Spawning potion @ {},{}", pos.x, pos.y);
    ecs.push(
        (
            Item,
            Potion,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('!')
            },
            Name("Potion".to_string())
        )
    );
}