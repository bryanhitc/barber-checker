#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod barber;

use anyhow::{Error, Result};
use barber::{Appointment, ResponseData};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder,
};

const URL: &str = "https://squareup.com/appointments/api/buyer/availability";
const STATIC_HEADERS: [(&str, &str); 4] = [
    ("content-type", "application/json"),
    ("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36"),
    ("accept", "*/*"),
    ("accept-encoding", "gzip, deflate, br"),
];

struct Settings {
    pub cookie: String,
    pub token: String,
    pub body: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let settings = Settings::from_args_or_env()?;
    let client = build_client(&settings)?;

    let appointments = {
        let response = client
            .post(URL)
            .body(settings.body)
            .send()
            .await?
            .error_for_status()?;

        let response_data = response.json::<ResponseData>().await?;

        let mut appointments = response_data
            .availability
            .iter()
            .filter(|a| a.available)
            .enumerate()
            .map(|(i, a)| Appointment::new(i + 1, a))
            .collect::<Vec<_>>();

        appointments.sort_unstable_by_key(|a| a.start);
        appointments
    };

    for appointment in &appointments {
        println!("{}", appointment);
    }

    let earliest_appointment = appointments.first();

    match earliest_appointment {
        Some(a) => println!("\nBest option is {}", a),
        None => println!("\nNo appointments found."),
    }

    Ok(())
}

fn build_client(settings: &Settings) -> Result<Client> {
    let mut headers = HeaderMap::with_capacity(STATIC_HEADERS.len() + 2);
    let static_headers = STATIC_HEADERS.iter().map(|(key, value)| {
        (
            HeaderName::from_static(key),
            HeaderValue::from_str(value).expect("parsing the static header values won't fail"),
        )
    });

    headers.extend(static_headers);
    headers.append("cookie", HeaderValue::from_str(&settings.cookie)?);
    headers.append("x-csrf-token", HeaderValue::from_str(&settings.token)?);

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .map_err(Error::from)
}

impl Settings {
    pub fn from_args_or_env() -> Result<Self> {
        // First arg doesn't matter since it's always the name of this binary
        let mut args = std::env::args().skip(1);

        let cookie = args
            .next()
            .map_or_else(|| std::env::var("BARBER_COOKIE"), Ok)?;

        let token = args
            .next()
            .map_or_else(|| std::env::var("BARBER_TOKEN"), Ok)?;

        let body = args
            .next()
            .map_or_else(|| std::env::var("BARBER_BODY"), Ok)?;

        Ok(Self {
            cookie,
            token,
            body,
        })
    }
}
