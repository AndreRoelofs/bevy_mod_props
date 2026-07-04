//! Tests that props types compose with bevy's `bsn!` scene notation.

use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::AssetPlugin;
use bevy_ecs::prelude::*;
use bevy_mod_props::prelude::*;
use bevy_scene::{ScenePlugin, prelude::*};

/// Creates an app with the minimal plugins needed to spawn scenes.
fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        TaskPoolPlugin::default(),
        AssetPlugin::default(),
        ScenePlugin,
    ));
    app
}

/// `EntityProps` is `Default + Clone`, so it qualifies as a pass-through
/// [`bevy_ecs::template::Template`] without any extra derives.
#[test]
fn entity_props_spawns_with_defaults() {
    let mut app = test_app();
    let world = app.world_mut();
    let entity = world.spawn_scene(bsn! { EntityProps }).unwrap().id();

    assert!(world.entity(entity).contains::<EntityProps>());
    assert_eq!(world.entity(entity).props().iter().count(), 0);
}

#[test]
fn entity_props_accepts_expressions() {
    let mut app = test_app();
    let world = app.world_mut();
    let entity = world
        .spawn_scene(bsn! {
            EntityProps({Props::new().with("health", 100.0).with("name", "frodo")})
        })
        .unwrap()
        .id();

    assert_eq!(world.entity(entity).get_prop::<f32>("health"), 100.0);
    assert_eq!(world.entity(entity).get_prop::<Estr>("name"), "frodo");
}

#[test]
fn entity_props_work_in_scene_hierarchies() {
    let mut app = test_app();
    let world = app.world_mut();
    let root = world
        .spawn_scene(bsn! {
            EntityProps({Props::new().with("depth", 0.0)})
            Children [
                EntityProps({Props::new().with("depth", 1.0)})
            ]
        })
        .unwrap()
        .id();

    assert_eq!(world.entity(root).get_prop::<f32>("depth"), 0.0);
    let child = world.entity(root).get::<Children>().unwrap()[0];
    assert_eq!(world.entity(child).get_prop::<f32>("depth"), 1.0);
}

/// When scenes are composed, a later `EntityProps` patch replaces the whole
/// property map; props from the base scene are not merged key-by-key.
#[test]
fn composed_scenes_replace_props_wholesale() {
    fn base() -> impl Scene {
        bsn! { EntityProps({Props::new().with("health", 100.0)}) }
    }

    let mut app = test_app();
    let world = app.world_mut();
    let entity = world
        .spawn_scene(bsn! {
            base()
            EntityProps({Props::new().with("mana", 50.0)})
        })
        .unwrap()
        .id();

    assert_eq!(world.entity(entity).get_prop::<f32>("mana"), 50.0);
    assert_eq!(world.entity(entity).get_prop::<f32>("health"), 0.0);
}
