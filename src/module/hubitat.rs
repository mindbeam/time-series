use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use futures::StreamExt;
pub use futures_timer::Delay;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, time::Duration};
use tempfile::TempDir;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::store::Store;

pub struct Hubitat {
    url: Url,
    store: Store,
}

impl Hubitat {
    pub fn new(connect_addr: String, store: Store) -> Self {
        let url = url::Url::parse(&connect_addr)
            .unwrap()
            .join("/eventsocket")
            .unwrap();
        println!("{}", url);

        Hubitat { url, store }
    }
    pub async fn run(&self) {
        loop {
            println!("Connecting to Hubitat...");
            match connect_async(&self.url).await {
                Ok((ws_stream, _)) => {
                    println!("WebSocket handshake has been successfully completed");

                    let (write, read) = ws_stream.split();

                    println!("Connected to Hubitat");

                    read.for_each(|message| async {
                        match message {
                            Ok(m) => {
                                let data = m.into_data();
                                if data.len() > 0 {
                                    self.store.add_event(data);
                                }
                            }
                            Err(e) => println!("Hubitat Message Error: {:?}", e),
                        }
                    })
                    .await;
                }
                Err(e) => {
                    println!("Hubitat Connect Error: {:?}", e);
                }
            }

            Delay::new(Duration::from_secs(2)).await
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DTO {
    pub source: String,
    pub name: String,
    pub display_name: String,
    pub value: String,
    pub unit: String,
    pub device_id: u32,
    pub hub_id: u32,
    pub installed_app_id: u32,
    pub description_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TempUnit {
    F,
    C,
}

impl TempUnit {
    fn from_str(s: &str) -> Self {
        match s {
            "°F" | "F" => Self::F,
            "°C" | "C" => Self::C,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Presence {
    Present,
    NotPresent,
}

impl From<String> for Presence {
    fn from(value: String) -> Self {
        match &value[..] {
            "present" => Presence::Present,
            _ => Presence::NotPresent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PressureUnit {
    kPa,
    PSI,
}

impl From<String> for PressureUnit {
    fn from(value: String) -> Self {
        match &value[..] {
            "kPa" => PressureUnit::kPa,
            "PSI" => PressureUnit::PSI,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum EventPayload {
    DeviceBattery { percent: u8 },
    DeviceCoolingSetpoint { degrees: f32, unit: TempUnit },
    DeviceHumidity { rh: f32 },
    DeviceLastCheckin(DateTime<FixedOffset>),
    DeviceLastCheckinEpoch(DateTime<Utc>),
    DevicePower,
    DevicePresence(Presence),
    DevicePressure { pressure: f32, unit: PressureUnit },
    DeviceStatus,
    DeviceSwitch,
    DeviceTemperature,
    DeviceThermostatOperatingState,
    DeviceThermostatSetpoint,
    LocationSunrise,
    LocationSunriseTime,
    LocationSunset,
    LocationSunsetTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    date: DateTime<Utc>,
    device_name: String,
    device_id: u32,
    hub_id: u32,
    payload: EventPayload,
}

impl Event {
    pub fn from_dto(dto: DTO) -> Option<Self> {
        let payload = match (&dto.source[..], &dto.name[..]) {
            ("DEVICE", "battery") => EventPayload::DeviceBattery {
                percent: dto.value.parse().unwrap(),
            },
            ("DEVICE", "coolingSetpoint") => EventPayload::DeviceCoolingSetpoint {
                degrees: dto.value.parse().unwrap(),
                unit: TempUnit::from_str(&dto.unit[..]),
            },
            ("DEVICE", "humidity") => EventPayload::DeviceHumidity {
                rh: dto.value.parse().unwrap(),
            },
            ("DEVICE", "lastCheckin") => EventPayload::DeviceLastCheckin(
                DateTime::parse_from_str(&dto.value, "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            ("DEVICE", "lastCheckinEpoch") => {
                EventPayload::DeviceLastCheckinEpoch(Utc.timestamp(dto.value.parse().unwrap(), 0))
            }
            ("DEVICE", "power") => EventPayload::DevicePower,
            ("DEVICE", "presence") => EventPayload::DevicePresence(dto.value.into()),
            ("DEVICE", "pressure") => EventPayload::DevicePressure {
                pressure: dto.value.parse().unwrap(),
                unit: dto.unit.into(),
            },
            ("DEVICE", "status") => EventPayload::DeviceStatus,
            ("DEVICE", "switch") => EventPayload::DeviceSwitch,
            ("DEVICE", "temperature") => EventPayload::DeviceTemperature,
            ("DEVICE", "thermostatOperatingState") => EventPayload::DeviceThermostatOperatingState,
            ("DEVICE", "thermostatSetpoint") => EventPayload::DeviceThermostatSetpoint,
            ("LOCATION", "sunrise") => EventPayload::LocationSunrise,
            ("LOCATION", "sunriseTime") => EventPayload::LocationSunriseTime,
            ("LOCATION", "sunset") => EventPayload::LocationSunset,
            ("LOCATION", "sunsetTime") => EventPayload::LocationSunsetTime,
            _ => return None,
        };

        let ev = Event {
            payload,
            date: Utc::now(),
            device_name: dto.display_name,
            device_id: dto.device_id,
            hub_id: dto.hub_id,
        };

        Some(ev)
    }
}
