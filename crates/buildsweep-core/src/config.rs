use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InactivityThreshold {
    #[serde(rename = "days_30")]
    Days30,
    #[serde(rename = "days_90")]
    Days90,
    #[serde(rename = "days_180")]
    Days180,
    #[serde(rename = "days_365")]
    Days365,
}

impl Default for InactivityThreshold {
    fn default() -> Self {
        Self::Days90
    }
}

impl InactivityThreshold {
    pub fn days(self) -> u32 {
        match self {
            Self::Days30 => 30,
            Self::Days90 => 90,
            Self::Days180 => 180,
            Self::Days365 => 365,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    pub scan_locations: Vec<String>,
    pub exclusions: Vec<String>,
    pub inactivity_threshold: InactivityThreshold,
    pub appearance: Appearance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Appearance {
    System,
    Light,
    Dark,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scan_locations: Vec::new(),
            exclusions: Vec::new(),
            inactivity_threshold: InactivityThreshold::default(),
            appearance: Appearance::System,
        }
    }
}
