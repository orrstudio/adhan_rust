// Время молитв — запросы к API aladhan.com, кэш в SQLite
use chrono::{Datelike, Local};
use reqwest::blocking::Client;
use serde::Deserialize;

pub struct PrayerTimesManager {
    client: Client,
    city: String,
    country: String,
    method: u32,
}

#[derive(Debug, Deserialize)]
struct AladhanResponse {
    code: i32,
    data: AladhanData,
}

#[derive(Debug, Deserialize)]
struct AladhanData {
    timings: AladhanTimings,
}

#[derive(Debug, Deserialize)]
struct AladhanTimings {
    #[serde(rename = "Fajr")]
    fajr: String,
    #[serde(rename = "Sunrise")]
    sunrise: String,
    #[serde(rename = "Dhuhr")]
    dhuhr: String,
    #[serde(rename = "Asr")]
    asr: String,
    #[serde(rename = "Maghrib")]
    maghrib: String,
    #[serde(rename = "Isha")]
    isha: String,
    #[serde(rename = "Midnight")]
    midnight: String,
}

#[derive(Debug, Clone)]
pub struct PrayerTimes {
    pub midnight: String,
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}

impl PrayerTimesManager {
    pub fn new() -> Self {
        PrayerTimesManager {
            client: Client::new(),
            city: "baku".to_string(),
            country: "AZ".to_string(),
            method: 13,
        }
    }

    pub fn fetch_prayer_times(&self) -> Result<PrayerTimes, reqwest::Error> {
        let date = Local::now();
        let date_str = format!("{:02}-{:02}-{}", date.day(), date.month(), date.year());

        let url = format!(
            "https://api.aladhan.com/v1/timingsByCity/{}",
            date_str
        );

        let resp: AladhanResponse = self
            .client
            .get(&url)
            .query(&[("city", &self.city), ("country", &self.country), ("method", &self.method.to_string())])
            .send()?
            .json()?;

        if resp.code != 200 {
            return Ok(PrayerTimes {
                midnight: "--:--".to_string(),
                fajr: "--:--".to_string(),
                sunrise: "--:--".to_string(),
                dhuhr: "--:--".to_string(),
                asr: "--:--".to_string(),
                maghrib: "--:--".to_string(),
                isha: "--:--".to_string(),
            });
        }

        let t = resp.data.timings;
        Ok(PrayerTimes {
            midnight: t.midnight,
            fajr: t.fajr,
            sunrise: t.sunrise,
            dhuhr: t.dhuhr,
            asr: t.asr,
            maghrib: t.maghrib,
            isha: t.isha,
        })
    }
}