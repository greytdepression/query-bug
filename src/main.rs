use bevy::prelude::*;

fn main() {
    App::new()

        // We need a game loop for the problem to occur.
        .add_plugins(DefaultPlugins)

        // spawn some entities
        .insert_resource(MyEntities::default())
        .add_startup_system(spawn_entities)
        .add_system(query_system)
        .run();
}

#[derive(Default, Resource)]
struct MyEntities(Vec<Entity>);

#[derive(Debug, Component, PartialEq, Eq)]
struct IndexComponent(i32); // we use this component to check that we have the correct component

#[derive(Component)]
#[component(storage = "SparseSet")]
struct MySparseComponent;


fn spawn_entities(mut commands: Commands, mut my_entities: ResMut<MyEntities>) {
    (0..10)
        .for_each(|i|
            // spawn an entity, give it an `IndexComponent` with its number, and push it into `MyEntities`
            my_entities.0.push(commands.spawn(IndexComponent(i)).id())
        );
}


fn query_system(
    mut commands: Commands,
    my_entities: Res<MyEntities>,
    query: Query<&IndexComponent>,
    mut frame_counter: Local<usize>,
) {

    let mut last: Option<Entity> = None;

    for &entity in my_entities.0.iter() {
        let get = query.get(entity).unwrap();
        let get_comp = query.get_component::<IndexComponent>(entity).unwrap();

        // give the entity the sparse component
        commands.entity(entity)
            .insert(MySparseComponent);

        // remove the sparse component from the last entity
        if let Some(last) = last {
            commands.entity(last)
                .remove::<MySparseComponent>();
        }

        // this is where things go wrong
        if get != get_comp {
            dbg!(entity, get, get_comp);
            dbg!(*frame_counter);
            panic!("AAAA");
        }

        last = Some(entity);
    }

    // count what frame we are on
    *frame_counter += 1;
}