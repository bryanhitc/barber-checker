mod barber;

use anyhow::Result;
use barber::{Appointment, ResponseData};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    ClientBuilder,
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

#[tokio::main]
async fn main() -> Result<()> {
    let settings = load_settings()?;
    let headers = generate_headers(&settings.cookie, &settings.token)?;

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("client should be valid");

    let response = client
        .post(URL)
        .body(settings.body)
        .send()
        .await?
        .error_for_status()?;

    let response_data = response.json::<ResponseData>().await?;

    let appointments = response_data
        .availability
        .into_iter()
        .filter(|a| a.available)
        .enumerate()
        .map(|(i, a)| Appointment::new(i + 1, a))
        .collect::<Vec<_>>();

    appointments.iter().for_each(|a| println!("{}", a));

    let earliest_appointment = appointments
        .iter()
        .min_by(|x, y| x.start.cmp(&y.start));

    match earliest_appointment {
        Some(a) => println!("\nBest option is {}", a),
        None => println!("\nNo appointments found."),
    }

    Ok(())
}

fn load_settings() -> Result<Settings> {
    // First arg doesn't matter since it's always the name of this binary
    let mut args = std::env::args().skip(1);

    let cookie = args
        .next()
        .map_or_else(|| std::env::var("BARBER_COOKIE"), Ok)
        .map_err(|err| anyhow::anyhow!(err))?;

    let token = args
        .next()
        .map_or_else(|| std::env::var("BARBER_TOKEN"), Ok)
        .map_err(|err| anyhow::anyhow!(err))?;

    let body = args
        .next()
        .map_or_else(|| std::env::var("BARBER_BODY"), Ok)
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(Settings {
        cookie,
        token,
        body,
    })
}

fn generate_headers(cookie: &str, token: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::with_capacity(STATIC_HEADERS.len() + 2);
    let static_headers = STATIC_HEADERS.iter().map(|(key, value)| {
        (
            HeaderName::from_static(key),
            HeaderValue::from_str(value).expect("parsing the static header values won't fail"),
        )
    });

    headers.extend(static_headers);
    headers.append("cookie", HeaderValue::from_str(cookie)?);
    headers.append("x-csrf-token", HeaderValue::from_str(token)?);

    Ok(headers)
}
