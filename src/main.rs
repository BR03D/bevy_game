#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

fn main() {
    App::new()
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa { samples: 4 })
    // Set WindowDescriptor Resource to change title and size
    .insert_resource(WindowDescriptor {
        title: "Game!".to_string(),
        width: 1600.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
    let x = App::new();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
        // Plane
        /*commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
            material: materials.add(Color::rgb(0.4, 0.5, 0.3).into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..Default::default()
        });*/
        // Camera
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 25.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        // Light
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

        //spawn player
        commands.spawn_scene(asset_server.load("Tank4.glb#Scene0"));


}

#[derive(Default)]
struct Hero{
    pos:(usize,usize),
    entity:Option<Entity>,
}
