// src/lib.rs

pub mod world;

#[cfg(test)]
mod tests {
    use super::world::entity::Entity;
    use super::world::component::{Position, Velocity};
    use super::world::system::update_position;
    use std::collections::HashMap;

    #[test]
    fn test_update_position() {
        let entity1 = Entity::new(1);
        let entity2 = Entity::new(2);

        let mut entities = HashMap::new();
        entities.insert(entity1, (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 }));
        entities.insert(entity2, (Position { x: 1.0, y: 1.0 }, Velocity { dx: -1.0, dy: -1.0 }));

        let new_positions = update_position(&entities);

        assert_eq!(new_positions.get(&entity1), Some(&Position { x: 1.0, y: 1.0 }));
        assert_eq!(new_positions.get(&entity2), Some(&Position { x: 0.0, y: 0.0 }));
    }
}
