use chrono::{Date, DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug)]
pub enum Error {
    Strum(strum::ParseError),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseBool(std::str::ParseBoolError),
    ChronoParse(chrono::ParseError),
    UnknownType { source: String, name: String },
}
impl From<strum::ParseError> for Error {
    fn from(e: strum::ParseError) -> Self {
        Self::Strum(e)
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}
impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}
impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Self::ChronoParse(e)
    }
}
impl From<std::str::ParseBoolError> for Error {
    fn from(e: std::str::ParseBoolError) -> Self {
        Self::ParseBool(e)
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

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum TempUnit {
    #[strum(serialize = "°F")]
    F,
    #[strum(serialize = "°C")]
    C,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum PowerUnit {
    W,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Presence {
    Present,
    NotPresent,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum DeviceStatus {
    Online,
    Offline,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
pub enum PressureUnit {
    kPa,
    PSI,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
pub enum ThermostatOperatingState {
    Idle,
    Cooling,
    Heating,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SwitchState {
    On,
    Off,
}

#[derive(Debug, Serialize, Deserialize)]
enum EventPayload {
    DeviceBattery { percent: u8 },
    DeviceCoolingSetpoint { value: f32, unit: TempUnit },
    DeviceHumidity { rh: f32 },
    DeviceLastCheckin(NaiveDateTime),
    DeviceLastCheckinEpoch(DateTime<Utc>),
    DevicePower { value: f32, unit: PowerUnit },
    DevicePresence(Presence),
    DevicePressure { pressure: f32, unit: PressureUnit },
    DeviceStatus(DeviceStatus),
    DeviceSwitch { state: SwitchState },
    DeviceTemperature { value: f32, unit: TempUnit },
    DeviceThermostatOperatingState(ThermostatOperatingState),
    DeviceThermostatSetpoint { value: f32, unit: TempUnit },
    LocationSunrise(bool),
    LocationSunriseTime(DateTime<Utc>),
    LocationSunset(bool),
    LocationSunsetTime(DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    date: DateTime<Utc>,
    device_name: String,
    device_id: u32,
    hub_id: u32,
    payload: EventPayload,
}

impl Event {
    pub fn from_dto(dto: DTO) -> Result<Self, Error> {
        let payload = match (&dto.source[..], &dto.name[..]) {
            ("DEVICE", "battery") => EventPayload::DeviceBattery {
                percent: dto.value.parse()?,
            },
            ("DEVICE", "coolingSetpoint") => EventPayload::DeviceCoolingSetpoint {
                value: dto.value.parse()?,
                unit: dto.unit.parse()?,
            },
            ("DEVICE", "humidity") => EventPayload::DeviceHumidity {
                rh: dto.value.parse()?,
            },
            ("DEVICE", "lastCheckin") => EventPayload::DeviceLastCheckin(
                NaiveDateTime::parse_from_str(&dto.value, "%Y-%m-%d %H:%M:%S")?,
            ),
            ("DEVICE", "lastCheckinEpoch") => {
                EventPayload::DeviceLastCheckinEpoch(Utc.timestamp(dto.value.parse()?, 0))
            }
            ("DEVICE", "power") => EventPayload::DevicePower {
                value: dto.value.parse()?,
                unit: dto.unit.parse()?,
            },
            ("DEVICE", "presence") => EventPayload::DevicePresence(dto.value.parse()?),
            ("DEVICE", "pressure") => EventPayload::DevicePressure {
                pressure: dto.value.parse()?,
                unit: dto.unit.parse()?,
            },
            ("DEVICE", "status") => EventPayload::DeviceStatus(dto.value.parse()?),
            ("DEVICE", "switch") => EventPayload::DeviceSwitch {
                state: dto.value.parse()?,
            },
            ("DEVICE", "temperature") => EventPayload::DeviceTemperature {
                value: dto.value.parse()?,
                unit: dto.unit.parse()?,
            },
            ("DEVICE", "thermostatOperatingState") => {
                EventPayload::DeviceThermostatOperatingState(dto.value.parse()?)
            }
            ("DEVICE", "thermostatSetpoint") => EventPayload::DeviceThermostatSetpoint {
                value: dto.value.parse()?,
                unit: dto.unit.parse()?,
            },
            ("LOCATION", "sunrise") => EventPayload::LocationSunrise(dto.value.parse()?),
            ("LOCATION", "sunriseTime") => EventPayload::LocationSunriseTime(dto.value.parse()?),
            ("LOCATION", "sunset") => EventPayload::LocationSunset(dto.value.parse()?),
            ("LOCATION", "sunsetTime") => EventPayload::LocationSunsetTime(dto.value.parse()?),
            _ => {
                return Err(Error::UnknownType {
                    source: dto.source,
                    name: dto.name,
                })
            }
        };

        let ev = Event {
            payload,
            date: Utc::now(),
            device_name: dto.display_name,
            device_id: dto.device_id,
            hub_id: dto.hub_id,
        };

        Ok(ev)
    }
}
