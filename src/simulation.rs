// src/simulation.rs
use crate::models::{Device, PacketData};
use bevy::prelude::*;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::time::{Duration, Instant};

#[derive(Resource)]
pub struct TrafficSimulator {
    devices: Vec<Device>,
    rng: StdRng,
    last_packet_time: Instant,
}

impl TrafficSimulator {
    pub fn new() -> Self {
        let devices = vec![
            Device {
                address: "192.168.1.1".to_string(),
            },
            Device {
                address: "192.168.1.2".to_string(),
            },
            Device {
                address: "192.168.1.3".to_string(),
            },
            Device {
                address: "192.168.1.4".to_string(),
            },
        ];

        // Seed the RNG with a known value or from the system entropy
        let rng = StdRng::from_entropy();

        Self {
            devices,
            rng,
            last_packet_time: Instant::now(),
        }
    }

    pub fn generate_packet(&mut self) -> Option<PacketData> {
        let now = Instant::now();
        if now.duration_since(self.last_packet_time) >= Duration::from_millis(500) {
            self.last_packet_time = now;
            let src = self.devices.choose(&mut self.rng).unwrap().clone();
            let dst = self.devices.choose(&mut self.rng).unwrap().clone();

            let size = self.rng.gen_range(100..1500);
            let protocols = vec!["TCP", "UDP", "ICMP"];
            let protocol = protocols.choose(&mut self.rng).unwrap().to_string();

            // Simple anomaly detection: packets larger than 1400 bytes are anomalous
            let anomalous = size > 1400;

            Some(PacketData {
                timestamp: now.elapsed().as_secs_f64(),
                src_addr: src.address,
                dst_addr: dst.address,
                size,
                protocol,
                anomalous,
            })
        } else {
            None
        }
    }
}
