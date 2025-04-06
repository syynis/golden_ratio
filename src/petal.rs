use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::PI;

use crate::{flower::SeedSettings, Callback, FlowerComponent};

pub struct FlowerPetalPlugin;

impl Plugin for FlowerPetalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_systems);
        // .init_resource::<SeedSettings>()
        //   .add_systems(Startup, spawn_initial_flowers)
        //   .add_systems(Update, animate_flowers);
    }
}

#[derive(Component)]
pub struct FlowerPetal;

#[derive(Component)]
pub struct ResetFlowerPetals;

fn register_systems(world: &mut World) {
    let spawn_id = world.register_system(spawn_petals);
    world.spawn((Callback(spawn_id), ResetFlowerPetals));
}

pub fn spawn_petals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut settings: ResMut<SeedSettings>,
    flowers: Query<Entity, With<FlowerComponent>>,
) {
    for entity in flowers.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if settings.mesh_handle.is_none() {
        settings.mesh_handle = Some(Mesh2dHandle(
            meshes.add(Ellipse::new(5.0 * settings.radius, 10.0 * settings.radius)),
        ))
    }
    if settings.material_handle.is_none() {
        settings.material_handle = Some(materials.add(settings.color));
    }
    for i in 1..settings.amount + 1 {
        let angle = 2.0 * PI * settings.rotation * (i as f32);
        let x = angle.cos() * settings.distance;
        let y = angle.sin() * settings.distance;
        let rotation = Quat::from_rotation_z(angle + PI / 2.0); // Adjust by 90 degrees (PI/2) to align the petal
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(Ellipse::new(5.1 * settings.radius, 10.1 * settings.radius)),
                ),
                material: materials.add(Color::Srgba(bevy::color::palettes::css::GREEN)),
                transform: Transform::from_xyz(x, y, i as f32).with_rotation(rotation),
                ..Default::default()
            })
            .with_children(|c| {
                c.spawn(MaterialMesh2dBundle {
                    mesh: settings.mesh_handle.clone().unwrap(),
                    material: settings.material_handle.clone().unwrap(),
                    transform: Transform::from_xyz(0., 0., 0.5),
                    ..Default::default()
                });
            })
            .insert(FlowerPetal)
            .insert(FlowerComponent);
    }
}
