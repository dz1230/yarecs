pub mod pool;
pub mod scene;
pub mod entity;
pub mod component;
pub mod error;

#[derive(Debug, Default)]
struct Position;
#[derive(Debug, Default)]
struct Velocity;

// fn main() {
//     let mut main_scene = scene::Scene::new();
//     let entity = main_scene.create_entity();
//     main_scene.assign_default::<Position>(entity).unwrap();
//     main_scene.assign_default::<Velocity>(entity).unwrap();
//     let mut entity_count = 0;
//     for entity in main_scene.view::<(Position, (Velocity, ()))>() {
//         println!("{:?}", entity);
//         entity_count += 1;
//     }
//     println!("Entity count: {}", entity_count);
// }
