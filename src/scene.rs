
use std::{any::Any, collections::HashMap};

use crate::{component::get_type_id, entity::Entity, error::RecsError, pool::Pool};

const MAX_COMPONENTS: usize = 64;
type ComponentMask = u64;

pub struct EntityDescription {
    entity: Entity,
    component_mask: ComponentMask
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
            self.entities.push(EntityDescription {
                entity,
                component_mask: 0
            });
            entity
        } else {
            let version = self.entities[index as usize].entity.version();
            let entity = Entity::with_version(index, version);
            self.entities[index as usize] = EntityDescription {
                entity,
                component_mask: 0
            };
            entity
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        let index = entity.index();
        self.entities[index as usize].entity.invalidate();
        self.entities[index as usize].component_mask = 0;
        self.free_list.push(index);
    }

    pub fn assign<T: 'static>(&mut self, entity: Entity, new_component: T) -> Result<&mut T, RecsError> {
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign(entity, new_component))
    }

    pub fn assign_default<T: Default + 'static>(&mut self, entity: Entity) -> Result<&mut T, RecsError> {
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign_default(entity))
    }

    pub fn remove<T: 'static>(&mut self, entity: Entity) {
        self.get_pool_if_exists_mut::<T>().map(|p| p.free(entity));
    }

    pub fn get<T: 'static>(&self, entity: Entity) -> Option<&T>{
        self.get_pool_if_exists::<T>().map_or(None, |p| p.get(entity))
    }

    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_pool_if_exists_mut::<T>().map_or(None, |p| p.get_mut(entity))
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
