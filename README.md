# Yet Another Rust Entity Component System

yarecs is a simple and flexible implementation of an ecs in rust.

It's based on this article: [How to make a simple entity-component-system in C++](https://www.david-colson.com/2020/02/09/making-a-simple-ecs.html),
but I had to make some changes to port it to rust.

No trait bounds for component types.

No limit on the number of component types.

No unsafe, no macros.

Should be reasonably fast and memory-efficient, but I haven't profiled it (yet).

Example code:

```rust
use yarecs::scene;

#[derive(Debug, Default)]
struct Position;
#[derive(Debug, Default)]
struct Velocity;

fn main() {
    let mut main_scene = scene::Scene::new();
    let entity = main_scene.create_entity();
    main_scene.assign_default::<Position>(entity).unwrap();
    main_scene.assign_default::<Velocity>(entity).unwrap();
    let mut entity_count = 0;
    for entity in main_scene.view::<(Position, (Velocity, ()))>() {
        println!("{:?}", entity);
        entity_count += 1;
    }
    println!("Entity count: {}", entity_count);
}
```