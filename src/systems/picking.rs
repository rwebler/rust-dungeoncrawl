use crate::prelude::*;

#[system]
#[read_component(PikeOfDestiny)]
#[read_component(WantsToPick)]
#[write_component(Health)]
#[write_component(Player)]
pub fn picking(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut pickers = <(Entity, &WantsToPick)>::query();
    let mut ecs2 = ecs.clone();
    let picks: Vec<(Entity, Entity, Entity)> = pickers
        .iter(ecs)
        .map(|(entity, pick)| (*entity, pick.picker, pick.object) )
        .collect();
    picks.iter().for_each(|(message, picker, object)| {
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
        commands.remove(*message);
    });
}