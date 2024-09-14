
use std::{any::Any, collections::{HashMap, HashSet}};

use crate::{component::get_type_id, entity::Entity, error::RecsError, pool::Pool};

pub struct EntityDescription {
    entity: Entity,
    components: HashSet<usize>,
}

impl EntityDescription {
    pub fn new(entity: Entity) -> Self {
        EntityDescription {
            entity,
            components: HashSet::new()
        }
    }

    pub fn invalidate_entity(&mut self) {
        self.entity.invalidate();
        self.components.clear();
    }

    /// Checks if this entity is valid (Has a valid id and has the same version as the given entity)
    pub fn check_validity(&self, entity: Entity) -> bool {
        self.entity == entity && self.entity.is_valid()
    }
}

pub struct Scene {
    /// List of free entity indices
    free_list: Vec<u32>,
    /// List of entities in the scene
    entities: Vec<EntityDescription>,
    /// Map of component pools keyed by type ID
    pools: HashMap<usize, Box<dyn Any>>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            free_list: Vec::new(),
            entities: Vec::new(),
            pools: HashMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let free_index = self.free_list.pop();
        let index = free_index.unwrap_or(self.entities.len() as u32);

        if free_index.is_none() {
            let entity = Entity::new(index);
            self.entities.push(EntityDescription::new(entity));
            entity
        } else {
            let version = self.entities[index as usize].entity.version();
            let entity = Entity::with_version(index, version);
            self.entities[index as usize] = EntityDescription::new(entity);
            entity
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        let index = entity.index();
        self.entities[index as usize].invalidate_entity();
        self.free_list.push(index);
    }

    pub fn assign<T: 'static>(&mut self, entity: Entity, new_component: T) -> Result<&mut T, RecsError> {
        let entity_description = self.get_valid_entity_description_mut(entity)?;
        entity_description.components.insert(get_type_id::<T>());
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign(entity, new_component))
    }

    pub fn assign_default<T: Default + 'static>(&mut self, entity: Entity) -> Result<&mut T, RecsError> {
        let entity_description = self.get_valid_entity_description_mut(entity)?;
        entity_description.components.insert(get_type_id::<T>());
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign_default(entity))
    }

    pub fn remove<T: 'static>(&mut self, entity: Entity) -> Result<(), RecsError> {
        self.get_valid_entity_description_mut(entity)
            .map(|ed| ed.components.remove(&get_type_id::<T>()))?;
        self.get_pool_if_exists_mut::<T>()
            .map(|p| p.free(entity));
        Ok(())
    }

    pub fn get<T: 'static>(&self, entity: Entity) -> Result<Option<&T>, RecsError> {
        self.assert_entity_valid(entity)?;
        Ok(self.get_pool_if_exists::<T>().map_or(None, |p| p.get(entity)))
    }

    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Result<Option<&mut T>, RecsError> {
        self.assert_entity_valid(entity)?;
        Ok(self.get_pool_if_exists_mut::<T>().map_or(None, |p| p.get_mut(entity)))
    }

    fn assert_entity_valid(&self, entity: Entity) -> Result<(), RecsError> {
        self.entities.get(entity.index() as usize)
            .filter(|ed| ed.check_validity(entity))
            .map_or(Err(RecsError::InvalidEntityError), |_| Ok(()))
    }

    fn get_valid_entity_description_mut(&mut self, entity: Entity) -> Result<&mut EntityDescription, RecsError> {
        self.entities.get_mut(entity.index() as usize)
            .filter(|ed| ed.check_validity(entity))
            .map_or(Err(RecsError::InvalidEntityError), Ok)
    }

    fn get_or_create_pool<T: 'static>(&mut self) -> Result<&mut Pool<T>, RecsError> {
        let type_id = get_type_id::<T>();
        self.pools
            .entry(type_id)
            .or_insert_with(|| Box::new(Pool::<T>::new()))
            .downcast_mut()
            .ok_or(RecsError::PoolAccessOrCreationError)
    }

    fn get_pool_if_exists<T: 'static>(&self) -> Option<&Pool<T>> {
        self.pools.get(&get_type_id::<T>())
            .and_then(|p| p.downcast_ref::<Pool<T>>())
    }

    fn get_pool_if_exists_mut<T: 'static>(&mut self) -> Option<&mut Pool<T>> {
        self.pools.get_mut(&get_type_id::<T>())
            .and_then(|p| p.downcast_mut::<Pool<T>>())
    }
}
