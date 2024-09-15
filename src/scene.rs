use std::{
    any::Any,
    collections::{HashMap, HashSet},
};

use crate::{component::{get_type_id, RequireComponents}, entity::Entity, error::RecsError, pool::Pool};

/// Holds an entity and a set of associated component types.
#[derive(Debug)]
pub struct EntityDescription {
    entity: Entity,
    components: HashSet<usize>,
}

impl EntityDescription {
    /// Creates a new entity description with the given entity and no associated components.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::entity::Entity;
    /// use recs::scene::EntityDescription;
    /// 
    /// let entity = Entity::new(0);
    /// let entity_description = EntityDescription::new(entity);
    /// 
    /// assert_eq!(entity_description.entity(), entity);
    /// assert!(entity_description.components().is_empty());
    /// ```
    pub fn new(entity: Entity) -> Self {
        EntityDescription {
            entity,
            components: HashSet::new(),
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn components(&self) -> &HashSet<usize> {
        &self.components
    }

    /// Invalidates the entity and clears the component list
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::entity::Entity;
    /// use recs::scene::EntityDescription;
    /// 
    /// let entity = Entity::new(0);
    /// let mut entity_description = EntityDescription::new(entity);
    /// entity_description.invalidate_entity();
    /// 
    /// assert!(!entity_description.entity().is_valid());
    /// assert!(entity_description.components().is_empty());
    /// ```
    pub fn invalidate_entity(&mut self) {
        self.entity.invalidate();
        self.components.clear();
    }

    /// Checks if this entity is valid.
    /// An entity description is valid if the entity is valid and has the same version as the given entity.
    ///
    /// # Example
    /// 
    /// ```
    /// use recs::entity::Entity;
    /// use recs::scene::EntityDescription;
    /// 
    /// let mut entity = Entity::new(0);
    /// let entity_description = EntityDescription::new(entity);
    /// assert!(entity_description.check_validity(entity));
    /// 
    /// entity.invalidate();
    /// assert!(!entity_description.check_validity(entity));
    /// ```
    pub fn check_validity(&self, entity: Entity) -> bool {
        self.entity == entity && self.entity.is_valid()
    }

    fn has_component_with_type_id(&self, type_id: usize) -> bool {
        self.components.contains(&type_id)
    }
}

/// A scene is a container for entities and their associated components.
#[derive(Debug)]
pub struct Scene {
    /// List of free entity indices
    free_list: Vec<u32>,
    /// List of entities in the scene
    entities: Vec<EntityDescription>,
    /// Map of component pools keyed by type ID
    pools: HashMap<usize, Box<dyn Any>>,
}

impl Scene {
    /// Creates a new empty scene.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let scene = Scene::new();
    /// ```
    pub fn new() -> Self {
        Scene {
            free_list: Vec::new(),
            entities: Vec::new(),
            pools: HashMap::new(),
        }
    }

    /// Creates a new entity and adds it to the scene.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// 
    /// let all_entities = scene.view::<()>().collect::<Vec<_>>();
    /// assert_eq!(all_entities.len(), 1);
    /// assert_eq!(all_entities[0], entity);
    /// ```
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

    /// Destroys an entity and removes it from the scene.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.destroy_entity(entity);
    /// 
    /// let all_entities = scene.view::<()>().collect::<Vec<_>>();
    /// assert!(all_entities.is_empty());
    /// ```
    pub fn destroy_entity(&mut self, entity: Entity) {
        let index = entity.index();
        self.entities[index as usize].invalidate_entity();
        self.free_list.push(index);
    }

    /// Assigns a component to an entity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign(entity, 42).unwrap();
    /// 
    /// let value = scene.get::<i32>(entity).unwrap().unwrap();
    /// assert_eq!(*value, 42);
    /// ```
    pub fn assign<T: 'static>(
        &mut self,
        entity: Entity,
        new_component: T,
    ) -> Result<&mut T, RecsError> {
        let entity_description = self.get_valid_entity_description_mut(entity)?;
        entity_description.components.insert(get_type_id::<T>());
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign(entity, new_component))
    }

    /// Assigns the default value of a component to an entity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign_default::<i32>(entity).unwrap();
    /// 
    /// let value = scene.get::<i32>(entity).unwrap().unwrap();
    /// assert_eq!(*value, 0);
    /// ```
    pub fn assign_default<T: Default + 'static>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut T, RecsError> {
        let entity_description = self.get_valid_entity_description_mut(entity)?;
        entity_description.components.insert(get_type_id::<T>());
        println!("{:?}", entity_description.components);
        let pool = self.get_or_create_pool::<T>()?;
        Ok(pool.assign_default(entity))
    }

    /// Removes a component from an entity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign(entity, 42).unwrap();
    /// scene.remove::<i32>(entity).unwrap();
    /// 
    /// let value = scene.get::<i32>(entity).unwrap();
    /// assert!(value.is_none());
    /// ```
    pub fn remove<T: 'static>(&mut self, entity: Entity) -> Result<(), RecsError> {
        self.get_valid_entity_description_mut(entity)
            .map(|ed| ed.components.remove(&get_type_id::<T>()))?;
        self.get_pool_if_exists_mut::<T>().map(|p| p.free(entity));
        Ok(())
    }

    /// Gets a component from an entity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign(entity, 42).unwrap();
    /// 
    /// let value = scene.get::<i32>(entity).unwrap().unwrap();
    /// assert_eq!(*value, 42);
    /// ```
    pub fn get<T: 'static>(&self, entity: Entity) -> Result<Option<&T>, RecsError> {
        self.assert_entity_valid(entity)?;
        Ok(self
            .get_pool_if_exists::<T>()
            .map_or(None, |p| p.get(entity)))
    }

    /// Gets a mutable component from an entity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign(entity, 42).unwrap();
    /// 
    /// let value = scene.get_mut::<i32>(entity).unwrap().unwrap();
    /// *value = 43;
    /// 
    /// let new_value = scene.get::<i32>(entity).unwrap().unwrap();
    /// assert_eq!(*new_value, 43);
    /// ```
    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Result<Option<&mut T>, RecsError> {
        self.assert_entity_valid(entity)?;
        Ok(self
            .get_pool_if_exists_mut::<T>()
            .map_or(None, |p| p.get_mut(entity)))
    }

    /// Returns an iterator over all entities that have all the required components.
    /// 
    /// # Example
    /// 
    /// ```
    /// use recs::scene::Scene;
    /// 
    /// let mut scene = Scene::new();
    /// let entity = scene.create_entity();
    /// scene.assign_default::<i32>(entity).unwrap();
    /// scene.assign_default::<f32>(entity).unwrap();
    /// 
    /// let all_entities = scene.view::<(i32, (f32,()))>().collect::<Vec<_>>();
    /// assert_eq!(all_entities.len(), 1);
    /// 
    /// let entities_with_i32 = scene.view::<(i32,())>().collect::<Vec<_>>();
    /// assert_eq!(entities_with_i32.len(), 1);
    /// 
    /// let entities_with_String = scene.view::<(String,())>().collect::<Vec<_>>();
    /// assert!(entities_with_String.is_empty());
    /// 
    /// let entities_with_i32_and_f32 = scene.view::<(i32, (f32,()))>().collect::<Vec<_>>();
    /// assert_eq!(entities_with_i32_and_f32.len(), 1);
    /// 
    /// let entities_with_String_and_f32 = scene.view::<(String, (f32,()))>().collect::<Vec<_>>();
    /// assert!(entities_with_String_and_f32.is_empty());
    /// ```
    pub fn view<'a, T: RequireComponents>(&'a self) -> SceneView<'a> {
        SceneView::new(self, T::required_component_ids())
    }

    fn assert_entity_valid(&self, entity: Entity) -> Result<(), RecsError> {
        self.entities
            .get(entity.index() as usize)
            .filter(|ed| ed.check_validity(entity))
            .map_or(Err(RecsError::InvalidEntityError), |_| Ok(()))
    }

    fn get_valid_entity_description_mut(
        &mut self,
        entity: Entity,
    ) -> Result<&mut EntityDescription, RecsError> {
        self.entities
            .get_mut(entity.index() as usize)
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
        self.pools
            .get(&get_type_id::<T>())
            .and_then(|p| p.downcast_ref::<Pool<T>>())
    }

    fn get_pool_if_exists_mut<T: 'static>(&mut self) -> Option<&mut Pool<T>> {
        self.pools
            .get_mut(&get_type_id::<T>())
            .and_then(|p| p.downcast_mut::<Pool<T>>())
    }
}

/// An iterator over a subset of the entities in a scene.
pub struct SceneView<'a> {
    scene: &'a Scene,
    required_components: Vec<usize>,
    index: usize,
}

impl<'a> SceneView<'a> {
    fn new(scene: &'a Scene, required_components: Vec<usize>) -> Self {
        SceneView {
            scene,
            required_components,
            index: 0,
        }
    }

    fn entity_has_required_components(&self, entity_description: &EntityDescription) -> bool {
        self.required_components
            .iter()
            .all(|&id| entity_description.has_component_with_type_id(id))
    }
}

impl<'a> Iterator for SceneView<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.scene.entities.len() {
            let entity_description = &self.scene.entities[self.index];
            self.index += 1;

            if self.entity_has_required_components(entity_description)
                && entity_description.entity.is_valid()
            {
                return Some(entity_description.entity);
            }
        }
        None
    }
}
