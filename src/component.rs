use std::sync::atomic::{AtomicUsize, Ordering};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    // Static counter for generating unique type IDs
    static ref TYPE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    // Map of rust type ids to recs type ids
    static ref TYPE_ID_MAP: Mutex<HashMap<TypeId, usize>> = Mutex::new(HashMap::new());
}

/// Returns a unique ID for a type
/// 
/// # Example
/// 
/// ```
/// use recs::component::get_type_id;
/// 
/// let int_id = get_type_id::<i32>();
/// let string_id = get_type_id::<String>();
/// assert_ne!(int_id, string_id);
/// 
/// let int_id2 = get_type_id::<i32>();
/// assert_eq!(int_id, int_id2);
/// ```
pub fn get_type_id<T: 'static>() -> usize {
    let type_id = TypeId::of::<T>();

    let mut map = TYPE_ID_MAP.lock().unwrap();
    *map.entry(type_id).or_insert_with(|| TYPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Collects type ids from variadic-esque type parameters 
/// 
/// # Example
/// 
/// ```
/// use recs::component::RequireComponents;
/// use recs::component::get_type_id;
/// 
/// fn variadic_example<T: RequireComponents>() -> Vec<usize> {
///     T::required_component_ids()
/// }
/// 
/// let int_id = get_type_id::<i32>();
/// let string_id = get_type_id::<String>();
/// let bool_id = get_type_id::<bool>();
/// 
/// assert_eq!(
///     variadic_example::<(i32, (String, (bool, ())))>(), 
///     vec![int_id, string_id, bool_id]
/// );
/// 
/// assert_eq!(
///    variadic_example::<()>(),
///     Vec::new()
/// );
/// ```
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
