use bevy::asset::{Assets, Handle};
use bevy::math::vec3;
use bevy::prelude::{ColorMaterial, Commands, default, Mesh, Res, ResMut, Resource, shape, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Resource)]
pub struct Circles {
    pub unit: Mesh2dHandle,
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let handle: Mesh2dHandle = meshes.add(shape::Circle::new(1.0).into()).into();
    commands.insert_resource(Circles { unit: handle });
}

pub fn mesh(
    circles: &Res<Circles>,
    material: &Handle<ColorMaterial>,
    radius: f32,
    x: f32, y: f32, z: f32,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: circles.unit.clone(),
        material: material.clone(),
        transform: Transform {
            translation: vec3(x, y, z),
            scale: vec3(radius, radius, 1.0),
            ..default()
        },
        ..default()
    }
}