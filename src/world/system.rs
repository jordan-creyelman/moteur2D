use crate::world::component::{Position, Velocity};
use crate::world::entity::Entity;
use std::collections::HashMap;

pub fn update_position(entities: &HashMap<Entity, (Position, Velocity)>) -> HashMap<Entity, Position> {
    let mut new_positions = HashMap::new();

    for (entity, (position, velocity)) in entities.iter() {
        let new_position = Position {
            x: position.x + velocity.dx,
            y: position.y + velocity.dy,
        };
        new_positions.insert(*entity, new_position);
    }

    new_positions
}
