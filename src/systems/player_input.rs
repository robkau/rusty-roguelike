use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub(crate) fn player_input(
    ecs: &mut SubWorld<'_>, // todo review https://doc.rust-lang.org/reference/lifetime-elision.html
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            players.iter(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;

                let _ = commands.push(((), WantsToMove{entity: *entity, destination}));
            });
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
