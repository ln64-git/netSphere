// src/main.rs
use bevy::prelude::*;

mod anomaly_detection;
mod models;
mod simulation;
mod visualization;

use models::PacketQueue;
use simulation::TrafficSimulator;
use visualization::VisualizationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(VisualizationPlugin)
        .insert_resource(TrafficSimulator::new())
        .insert_resource(PacketQueue::default())
        .add_systems(Update, simulation_system)
        .run();
}

fn simulation_system(
    mut simulator: ResMut<TrafficSimulator>,
    mut packet_queue: ResMut<PacketQueue>,
) {
    if let Some(mut packet) = simulator.generate_packet() {
        // Perform anomaly detection
        packet.anomalous = anomaly_detection::detect_anomaly(&packet);

        // Add packet to the queue for visualization
        packet_queue.packets.push(packet);
    }
}
