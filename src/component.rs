use std::sync::{Once, atomic::{AtomicUsize, Ordering}};

static TYPE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn get_type_id<T>() -> usize {
    static TYPE_ID: AtomicUsize = AtomicUsize::new(usize::MAX);
    static COMPONENT_INIT: Once = Once::new();

    COMPONENT_INIT.call_once(|| {
        let new_id = TYPE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        TYPE_ID.store(new_id, Ordering::Relaxed);
    });

    TYPE_ID.load(Ordering::Relaxed)
}