use crate::prelude::*;

#[system]
#[read_component(WantsToPick)]
#[read_component(Player)]
#[read_component(PikeOfDestiny)]
#[write_component(Health)]
pub fn picking(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut pickers = <(Entity, &WantsToPick)>::query();
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
        }
        commands.remove(*message);
    });
}