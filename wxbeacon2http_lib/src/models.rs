use chrono::{DateTime, Utc};
use serde::Serialize;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct WxBroadcastFormat {
    pub seq_no: u8,
    pub temperature: [u8; 2],
    pub humidity: [u8; 2],
    pub illuminance: [u8; 2],
    pub uv_index: [u8; 2],
    pub pressure: [u8; 2],
    pub noise: [u8; 2],
    pub discomfort_index: [u8; 2],
    pub wgbt: [u8; 2],
    pub _rfu: [u8; 2],
    pub battery_voltage: u8,
}

impl WxBroadcastFormat {
    pub fn parse_env_datum(&self) -> EnvDatum {
        EnvDatum {
            timestamp: Utc::now(),
            temperature: f64::from(i16::from_le_bytes(self.temperature)) * 0.01,
            humidity: f64::from(i16::from_le_bytes(self.humidity)) * 0.01,
            illuminance: u16::from_le_bytes(self.illuminance),
            uv_index: f64::from(i16::from_le_bytes(self.uv_index)) * 0.01,
            pressure: f64::from(i16::from_le_bytes(self.pressure)) * 0.1,
            noise: f64::from(i16::from_le_bytes(self.noise)) * 0.01,
            discomfort_index: f64::from(i16::from_le_bytes(self.discomfort_index)) * 0.01,
            wgbt: f64::from(i16::from_le_bytes(self.wgbt)) * 0.01,
            battery_voltage: f64::from(self.battery_voltage as u32 + 100) * 0.01,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EnvDatum {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
    pub illuminance: u16,
    pub uv_index: f64,
    pub pressure: f64,
    pub noise: f64,
    pub discomfort_index: f64,
    pub wgbt: f64,
    pub battery_voltage: f64,
}