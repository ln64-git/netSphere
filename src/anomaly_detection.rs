// src/anomaly_detection.rs
use crate::models::PacketData;

pub fn detect_anomaly(packet: &PacketData) -> bool {
    // Simple rule: packets larger than 1400 bytes are anomalous
    packet.size > 1400
}
