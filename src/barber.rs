use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Deserialize;

// --------------- Auto-generated structs ---------------

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    #[serde(rename = "staff_ids")]
    pub staff_ids: Vec<String>,
    pub availability: Vec<Availability>,
    pub resources: Resources,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    pub start: i64,
    pub end: i64,
    pub available: bool,
    #[serde(rename = "staff_id")]
    pub staff_id: String,
    pub segments: Vec<Segment>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub start: i64,
    pub end: i64,
    #[serde(rename = "resource_token")]
    pub resource_token: ::serde_json::Value,
    #[serde(rename = "employee_token")]
    pub employee_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {}

// ------------------------------------------------------

pub struct Appointment {
    pub id: usize,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl Appointment {
    pub fn new(id: usize, availability: Availability) -> Self {
        Self {
            id,
            start: local_from_timestamp(availability.start),
            end: local_from_timestamp(availability.end),
        }
    }
}

impl Display for Appointment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_format = "%D %r";
        write!(
            f,
            "Appointment #{}: {} - {}",
            self.id,
            self.start.format(date_format),
            self.end.format(date_format)
        )
    }
}

fn local_from_timestamp(timestamp: i64) -> DateTime<Local> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let utc = DateTime::<Utc>::from_utc(naive, Utc);
    utc.with_timezone(&Local)
}
