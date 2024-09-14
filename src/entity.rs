
// // Global mutable storage for entities
// static ENTITIES: OnceLock<Mutex<Vec<Entity>>> = OnceLock::new();

// fn get_entity_storage() -> &'static Mutex<Vec<Entity>> {
//     ENTITIES.get_or_init(|| Mutex::new(Vec::new()))
// }

// pub fn add_entity(entity: Entity) {
//     let mut entities = ENTITIES.lock().unwrap();
//     entities.push(entity);
// }

// pub fn get_entities() -> Vec<Entity> {
//     let entities = ENTITIES.lock().unwrap();
//     entities.clone()
// }

// invalid entity: index -1 version 0
// static INVALID_ENTITY: Entity = Entity::new(u32::MAX);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    version: u32,
    index: u32,
}

impl Entity {
    pub const fn new(index: u32) -> Self {
        Entity {
            version: 0,
            index,
        }
    }

    pub fn with_version(index: u32, version: u32) -> Self {
        Entity {
            version,
            index,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn invalidate(&mut self) {
        self.index = u32::MAX;
        self.version += 1;
    }

    pub fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}