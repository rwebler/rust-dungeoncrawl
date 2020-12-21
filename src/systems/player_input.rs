use crate::prelude::*;
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero()
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos+delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut items = <(Entity, &Point)>::query()
            .filter(component::<Item>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            victim: *entity,
                        }));
                });
            if !hit_something {
                items
                    .iter(ecs)
                    .filter(|(_, pos)| {
                        **pos == destination
                    })
                    .for_each(|(entity, _)| {
                        commands
                            .push(((), WantsToPick{
                                picker: player_entity,
                                object: *entity
                            }));
                    });
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}