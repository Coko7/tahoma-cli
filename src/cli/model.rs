use std::fmt;

use anyhow::anyhow;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub hostname: String,
    pub port: i32,
    // pod: String,
    pub api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceGroup {
    id: String,
    name: String,
    devices: Vec<String>,
}

impl DeviceGroup {
    pub fn new(name: &str) -> DeviceGroup {
        DeviceGroup {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            devices: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }

    pub fn has_device(&self, device_id: &str) -> bool {
        self.devices.iter().any(|id| id == device_id)
    }

    pub fn add_device(&mut self, device_id: &str) -> Result<(), anyhow::Error> {
        if self.has_device(device_id) {
            return Err(anyhow!("Device already in group"));
        }

        Ok(self.devices.push(device_id.to_string()))
    }

    pub fn remove_device(&mut self, device_id: &str) -> Result<(), anyhow::Error> {
        if !self.has_device(device_id) {
            return Err(anyhow!("Device not in group"));
        }

        Ok(self.devices.retain(|id| id != device_id))
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MatchMode {
    Exact,
    Fuzzy,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceTypeFilter {
    All,
    GarageDoor,
    Gate,
    RollerShutter,
}

impl DeviceTypeFilter {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceTypeFilter::All => "",
            DeviceTypeFilter::GarageDoor => "io:GarageOpenerIOComponent",
            DeviceTypeFilter::Gate => "io:SlidingDiscreteGateOpenerIOComponent",
            DeviceTypeFilter::RollerShutter => "io:RollerShutterWithLowSpeedManagementIOComponent",
        }
    }
}

impl fmt::Display for DeviceTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
