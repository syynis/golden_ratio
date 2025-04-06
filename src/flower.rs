use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::PI;

use crate::{constants::PHI, ui::UiState, Callback, FlowerComponent};

pub struct FlowerSeedPlugin;

impl Plugin for FlowerSeedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SeedSettings>()
            .add_systems(Startup, (register_systems, spawn_flower_seeds))
            .add_systems(Update, animate_flower_seeds);
    }
}

#[derive(Component)]
pub struct FlowerSeed;

#[derive(Resource)]
pub struct SeedSettings {
    pub rotation: f32,
    pub distance: f32,
    pub radius: f32,
    pub amount: i32,
    pub color: Color,
    pub material_handle: Option<Handle<ColorMaterial>>,
    pub mesh_handle: Option<Mesh2dHandle>,
}

impl Default for SeedSettings {
    fn default() -> Self {
        Self {
            rotation: PHI.fract(),
            distance: 20.0,
            radius: 10.0,
            amount: 100,
            color: Color::Srgba(bevy::color::palettes::css::GREEN),
            material_handle: None,
            mesh_handle: None,
        }
    }
}

impl SeedSettings {
    pub fn default_petal() -> Self {
        Self {
            rotation: 0.,
            distance: 50.0,
            radius: 4.0,
            amount: 1,
            color: Color::Srgba(bevy::color::palettes::css::GREEN),
            material_handle: None,
            mesh_handle: None,
        }
    }
}

#[derive(Component)]
pub struct ResetFlowerSeeds;

fn register_systems(world: &mut World) {
    let spawn_id = world.register_system(spawn_flower_seeds);
    world.spawn((Callback(spawn_id), ResetFlowerSeeds));
}

/*
fn spawn_initial_flowers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    seed_settings: Res<SeedSettings>
) {
    spawn_flowers(&mut commands, &mut meshes, &mut materials, seed_settings.deref());
}*/

fn animate_flower_seeds(
    mut commands: Commands,
    mut seed_settings: ResMut<SeedSettings>,
    ui_state: Res<UiState>,
    time: Res<Time>,
    reset_seeds: Query<&Callback, With<ResetFlowerSeeds>>,
) {
    if !ui_state.animate {
        return;
    }
    seed_settings.rotation += ui_state.step_size * time.delta_seconds();
    commands.run_system(reset_seeds.single().0);
}

fn spawn_flower_seeds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut settings: ResMut<SeedSettings>,
    flowers: Query<Entity, With<FlowerComponent>>,
) {
    for e in &flowers {
        commands.entity(e).despawn_recursive();
    }
    if settings.mesh_handle.is_none() {
        settings.mesh_handle = Some(Mesh2dHandle(meshes.add(Circle {
            radius: settings.radius,
        })))
    }
    if settings.material_handle.is_none() {
        settings.material_handle = Some(materials.add(settings.color));
    }
    for i in 1..settings.amount + 1 {
        let angle = 2.0 * PI * settings.rotation * (i as f32);
        let radius = 2.0 * (i as f32).sqrt();
        let x = angle.cos() * radius * settings.distance;
        let y = angle.sin() * radius * settings.distance;
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: settings.mesh_handle.clone().unwrap().clone(),
                material: settings.material_handle.clone().unwrap(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            })
            .insert(FlowerSeed)
            .insert(FlowerComponent);
    }
}
