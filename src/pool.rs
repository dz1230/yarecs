use std::collections::HashMap;

use crate::entity::Entity;

pub struct Pool<T> {
    /// Map of entity id to component index
    entities: HashMap<u32, u32>,
    /// List of free component indices
    free_list: Vec<u32>,
    /// Vector of components in the pool
    components: Vec<T>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Pool {
            entities: HashMap::new(),
            free_list: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Pool {
            entities: HashMap::new(),
            free_list: Vec::new(),
            components: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.entities.get(&entity.index()).map(|&index| &self.components[index as usize])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.entities.get(&entity.index()).map(|&index| &mut self.components[index as usize])
    }

    pub fn assign(&mut self, entity: Entity, new_component: T) -> &mut T {
        // If anyone knows a better way to do this while still satisfying the borrow checker, please let me know
        if self.get(entity).is_some() {
            self.get_mut(entity).unwrap()
        } else {
            self.insert_new_component(entity, new_component)
        }
    }

    pub fn free(&mut self, entity: Entity) {
        if let Some(&index) = self.entities.get(&entity.index()) {
            self.free_list.push(index);
            self.entities.remove(&entity.index());
        }
    }

    fn insert_new_component(&mut self, entity: Entity, new_component: T) -> &mut T {
        let free_index = self.free_list.pop();
        let index = free_index.unwrap_or(self.components.len() as u32);

        if free_index.is_none() {
            self.components.push(new_component);
        } else {
            self.components[index as usize] = new_component;
        }

        self.entities.insert(entity.index(), index);
        &mut self.components[index as usize]
    }
}

impl <T: Default> Pool<T> {
    pub fn assign_default(&mut self, entity: Entity) -> &mut T {
        if self.get(entity).is_some() {
            self.get_mut(entity).unwrap()
        } else {
            self.insert_new_component(entity, Default::default())
        }
    }
}