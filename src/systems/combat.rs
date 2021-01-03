use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] kills: &mut Kills) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim, attack.attacker) )
        .collect();
    victims.iter().for_each(|(message, victim, attacker)| {
        let ecs2 = ecs.clone();
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            let mut damage = 1;
            if let Ok(player) = ecs2
                .entry_ref(*attacker)
                .unwrap()
                .get_component::<Player>()
            {
                damage = player.damage;
            }
            health.current -= damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
                *kills += 1;
            }
        }
        commands.remove(*message);
    });
}