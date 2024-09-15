use std::sync::atomic::{AtomicUsize, Ordering};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref TYPE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref TYPE_ID_MAP: Mutex<HashMap<TypeId, usize>> = Mutex::new(HashMap::new());
}

pub fn get_type_id<T: 'static>() -> usize {
    let type_id = TypeId::of::<T>();

    let mut map = TYPE_ID_MAP.lock().unwrap();
    *map.entry(type_id).or_insert_with(|| TYPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}

// Collects component type IDs
pub trait RequireComponents {
    fn required_component_ids() -> Vec<usize>;
}

// Recursion base case
impl RequireComponents for () {
    fn required_component_ids() -> Vec<usize> {
        Vec::new()
    }
}

// Recursion case
impl<Head, Tail> RequireComponents for (Head, Tail)
where
    Head: 'static,
    Tail: RequireComponents,
{
    fn required_component_ids() -> Vec<usize> {
        let mut ids = vec![get_type_id::<Head>()];
        ids.extend(Tail::required_component_ids());
        ids
    }
}
