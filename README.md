
# Bevy Toon Material

A work-in-progress toon material for the Bevy game engine.

![Toon material example](images/toon-shapes.png)

## Usage

Add plugin and use material:

```rust
use bevy::prelude::*;
use bevy_toon_material::{ToonMaterial, ToonMaterialPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ToonMaterialPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(ToonMaterial {
            color: Color::rgb(0.1, 0.1, 0.6),
            glossiness: 5.0,
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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
```

See the examples folder for more detail usage.

## Examples

```bash
$ cargo run --examples demo
```

## Roadmap

- Convert to library - IN PROGRESS

- Add base color texture

- Add outlines?

- Add normal map?


## Reference

- [Bevy](https://github.com/bevyengine/bevy)
- [Custom Toon Shader in Three.js](https://www.maya-ndljk.com/blog/threejs-basic-toon-shader)
- [Toon Shader using Unity engine](https://roystan.net/articles/toon-shader/)
