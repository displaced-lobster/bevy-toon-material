use bevy::prelude::*;
use bevy_spectator::{Spectator, SpectatorPlugin};
use bevy_toon_material::{ToonMaterial, ToonMaterialPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(ToonMaterialPlugin)
        .add_plugin(SpectatorPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::default())),
        transform: Transform::from_xyz(-3.0, 1.0, 0.0),
        material: materials.add(ToonMaterial {
            color: Color::rgb(0.6, 0.1, 0.1),
            glossiness: 1.0,
            ..default()
        }),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        material: materials.add(ToonMaterial {
            color: Color::rgb(0.1, 0.1, 0.6),
            glossiness: 5.0,
            ..default()
        }),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Torus::default())),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        material: materials.add(ToonMaterial {
            color: Color::rgb(0.1, 0.6, 0.1),
            glossiness: 5.0,
            ..default()
        }),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(ToonMaterial {
            color: Color::SILVER,
            ..default()
        }),
        ..default()
    });


    commands.spawn(PointLightBundle {
        point_light: PointLight {
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Spectator,
    ));
}
