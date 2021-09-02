use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod objects;
mod setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup::init.system())
        .add_startup_system(objects::tower::tower.system())
        //.add_plugin(InspectorPlugin::<Data>::new())
        .run();
}
