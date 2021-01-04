use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(PikeOfDestiny)]
pub fn hud(ecs: &SubWorld, #[resource] level: &Level, #[resource] kills: &Kills) {
    let mut player_query = <(&Point, &Health)>::query().filter(component::<Player>());
    let (player_pos, player_health) = player_query
        .iter(ecs)
        .nth(0)
        .unwrap()
    ;
    let mut draw_batch = DrawBatch::new();
    let level_text = match level {
        0 => "Explore the Forest. Find the entrance to the Dungeon",
        3 => "Explore the Dungeon. Find the Amulet",
        _ => "Delve deeper into the Dungeon",
    };
    let mut help_text = ".";
    let mut pike = <&Point>::query().filter(component::<PikeOfDestiny>());
    if let Some(_pike_pos) = pike.iter(ecs).next()
    {
        if *level < 3 && *level > 0 {
            help_text = ". Look out for the Pike.";
        }
    }
    draw_batch.target(2);
    draw_batch.print_centered(1, format!("{}{} Use cursor keys to move.", level_text, help_text));
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK)
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED)
    );
    draw_batch.print_color_centered(
        2,
        format!("Level: {} / {} ({},{}) Kills: {}", level, 3, player_pos.x, player_pos.y, kills),
        ColorPair::new(WHITE, BLACK)
    );
    draw_batch.submit(10000).expect("Batch error");
}