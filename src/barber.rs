use std::fmt::Display;

use jiff::{tz::TimeZone, Timestamp, ZonedDateTime};
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
    pub start: ZonedDateTime,
    pub end: ZonedDateTime,
}

impl Appointment {
    pub fn new(id: usize, availability: &Availability) -> Self {
        Self {
            id,
            start: local_from_timestamp(availability.start),
            end: local_from_timestamp(availability.end),
        }
    }
}

impl Display for Appointment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DATE_FORMAT: &str = "%D %r";
        write!(
            f,
            "Appointment #{}: {} - {}",
            self.id,
            self.start.strftime(DATE_FORMAT),
            self.end.strftime(DATE_FORMAT)
        )
    }
}

fn local_from_timestamp(timestamp: i64) -> ZonedDateTime {
    Timestamp::from_second(timestamp)
        .expect("timestamp should be in range")
        .to_zoned_datetime(TimeZone::system())
}
