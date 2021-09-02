use bevy::prelude::*;

pub fn tower(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tower_mesh: Handle<Mesh> =
        server.load("tower/source/building_02_fbx.FBX.gltf#Mesh0/Primitive0");
    let text = server.load("tower/textures/Building-02_albedo.png");

    let white_material = materials.add(StandardMaterial {
        base_color_texture: Some(text.clone()),
        ..Default::default()
    });
    commands
        // Spawn parent entity
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: tower_mesh.clone(),
                material: white_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}
