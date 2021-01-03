use crate::prelude::*;

#[system]
#[read_component(PikeOfDestiny)]
#[read_component(Potion)]
#[read_component(WantsToPick)]
#[write_component(Health)]
#[write_component(Player)]
pub fn picking(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut pickers = <(Entity, &WantsToPick)>::query();
    let picks: Vec<(Entity, Entity, Entity)> = pickers
        .iter(ecs)
        .map(|(entity, pick)| (*entity, pick.picker, pick.object) )
        .collect();
    picks.iter().for_each(|(message, picker, object)| {
        check_is_pike(ecs, commands, picker, object);
        check_is_potion(ecs, commands, picker, object);
        commands.remove(*message);
    });
}

fn check_is_pike(ecs: &mut SubWorld, commands: &mut CommandBuffer, picker: &Entity, object: &Entity) {
    let mut ecs2 = ecs.clone();
    if ecs
            .entry_ref(*object)
            .unwrap()
            .get_component::<PikeOfDestiny>()
            .is_ok()
        {
            if let Ok(mut health) = ecs
                .entry_mut(*picker)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = health.max;
                commands.remove(*object);
            }
            if let Ok(mut player) = ecs2
                .entry_mut(*picker)
                .unwrap()
                .get_component_mut::<Player>()
            {
                player.damage = 2;
            }
        }
}

fn check_is_potion(ecs: &mut SubWorld, commands: &mut CommandBuffer, picker: &Entity, object: &Entity) {
    use std::cmp::min;
    if ecs
            .entry_ref(*object)
            .unwrap()
            .get_component::<Potion>()
            .is_ok()
        {
            if let Ok(mut health) = ecs
                .entry_mut(*picker)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = min(health.current + 3, health.max);
                commands.remove(*object);
            }
        }
}