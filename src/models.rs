// src/models.rs
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Component)]
pub struct PacketData {
    pub timestamp: f64,
    pub src_addr: String,
    pub dst_addr: String,
    pub size: usize,
    pub protocol: String,
    pub anomalous: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Component)]
pub struct Device {
    pub address: String,
}

#[derive(Resource, Default)]
pub struct PacketQueue {
    pub packets: Vec<PacketData>,
}
