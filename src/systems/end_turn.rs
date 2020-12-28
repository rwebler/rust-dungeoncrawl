use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
#[read_component(Entrance)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let mut entrance = <&Point>::query().filter(component::<Entrance>());
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::Descend => TurnState::AwaitingInput,
        _ => current_state,
    };
    player.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if let Some(amulet_pos) = amulet.iter(ecs).nth(0) {
            if amulet_pos == pos {
                new_state = TurnState::Victory;
            }
        }
        if let Some(entrance_pos) = entrance.iter(ecs).nth(0) {
            if entrance_pos == pos {
                new_state = TurnState::Descend;
            }
        }
    });
    *turn_state = new_state;
}