#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity {
    pub id: u32,
}

impl Entity {
    // Création d'une nouvelle entité avec un identifiant auto-incrémenté
    pub fn new() -> Self {
        // Utilisation d'une variable statique mutable pour garder le dernier identifiant
        static mut NEXT_ID: u32 = 1;

        unsafe {
            let current_id = NEXT_ID;
            NEXT_ID += 1;
            Entity { id: current_id }
        }
    }
}
