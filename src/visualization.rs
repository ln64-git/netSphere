// src/visualization.rs
use bevy::prelude::*;
use std::collections::HashMap;

use crate::models::{Device, PacketData, PacketQueue};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DeviceMap::default())
            .add_systems(Startup, setup_system)
            .add_systems(Update, packet_visualization_system)
            .add_systems(Update, camera_controller_system);
    }
}

#[derive(Default, Resource)]
pub struct DeviceMap {
    pub devices: HashMap<String, Entity>,
}

fn setup_system(
    mut commands: Commands,
    mut device_map: ResMut<DeviceMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Setup camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Setup light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });

    // Create device entities
    let positions = vec![
        Vec3::new(-10.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(0.0, 0.0, 10.0),
    ];

    let device_addresses = vec![
        "192.168.1.1".to_string(),
        "192.168.1.2".to_string(),
        "192.168.1.3".to_string(),
        "192.168.1.4".to_string(),
    ];

    for (position, address) in positions.into_iter().zip(device_addresses.into_iter()) {
        let mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
        let material = materials.add(StandardMaterial {
            base_color: Color::BLUE,
            ..default()
        });

        let entity = commands
            .spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Device {
                address: address.clone(),
            })
            .id();

        device_map.devices.insert(address, entity);
    }
}

fn packet_visualization_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    device_map: Res<DeviceMap>,
    mut packet_queue: ResMut<PacketQueue>,
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut Transform, &PacketData)>,
        Query<&Transform>,
    )>,
) {
    // Update existing packets
    for (entity, mut transform, _packet_data) in param_set.p0().iter_mut() {
        // Compute the forward vector and delta movement
        let forward = transform.forward();
        let delta = forward * time.delta_seconds() * 5.0;

        // Compute the new translation
        let new_translation = transform.translation + delta;

        // Check if the packet has moved far enough
        if new_translation.distance(Vec3::ZERO) > 50.0 {
            commands.entity(entity).despawn();
        } else {
            // Update the transform's translation
            transform.translation = new_translation;
        }
    }

    // Spawn new packets
    if let Some(packet) = packet_queue.packets.pop() {
        if let (Some(&src_entity), Some(&dst_entity)) = (
            device_map.devices.get(&packet.src_addr),
            device_map.devices.get(&packet.dst_addr),
        ) {
            // Bind the query to extend its lifetime
            let transform_query = param_set.p1();

            // Get the source and destination translations
            let src_translation = transform_query.get(src_entity).unwrap().translation;

            let dst_translation = transform_query.get(dst_entity).unwrap().translation;

            // Compute the direction
            let direction = (dst_translation - src_translation).normalize();

            let mesh = meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 32,
                stacks: 16,
            }));

            let material = materials.add(StandardMaterial {
                base_color: if packet.anomalous {
                    Color::RED
                } else {
                    Color::GREEN
                },
                ..default()
            });

            commands
                .spawn(PbrBundle {
                    mesh,
                    material,
                    transform: Transform {
                        translation: src_translation,
                        rotation: Quat::from_rotation_arc(Vec3::Z, direction),
                        scale: Vec3::splat(0.5),
                    },
                    ..default()
                })
                .insert(packet);
        }
    }
}

use bevy::input::mouse::{MouseMotion, MouseWheel};

fn camera_controller_system(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut translation = Vec3::ZERO;
    let mut rotation = Quat::IDENTITY;

    // Handle keyboard input for movement
    if keyboard_input.pressed(KeyCode::W) {
        translation.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        translation.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        translation.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        translation.x += 1.0;
    }

    // Handle mouse input for rotation
    for event in mouse_motion_events.iter() {
        let delta = event.delta;
        let yaw = Quat::from_rotation_y(-delta.x * 0.005);
        let pitch = Quat::from_rotation_x(-delta.y * 0.005);
        rotation = yaw * pitch;
    }

    // Handle mouse wheel for zoom
    for event in mouse_wheel_events.iter() {
        translation.z += event.y;
    }

    for mut transform in query.iter_mut() {
        transform.translation += translation * time.delta_seconds() * 10.0;
        transform.rotation = rotation * transform.rotation;
    }
}
