use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct IpInfo {
    pub ip: String,
    #[serde(rename = "type")]
    pub ip_type: String,
    pub continent_code: String,
    pub continent_name: String,
    pub country_code: String,
    pub country_name: String,
    pub region_code: String,
    pub region_name: String,
    pub city: String,
    pub zip: String,
    pub latitude: f64,
    pub longitude: f64,
    pub location: Location,
    pub time_zone: TimeZone,
    pub currency: Currency,
    pub connection: Connection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub geoname_id: u32,
    pub capital: String,
    pub languages: Vec<Language>,
    pub country_flag: String,
    pub country_flag_emoji: String,
    pub country_flag_emoji_unicode: String,
    pub calling_code: String,
    pub is_eu: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZone {
    pub id: String,
    pub current_time: String,
    pub gmt_offset: i32,
    pub code: String,
    pub is_daylight_saving: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub plural: String,
    pub symbol: String,
    pub symbol_native: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub asn: u32,
    pub isp: String,
}
use std::fmt;

impl fmt::Display for IpInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.city, self.region_name, self.country_name, self.zip
        )
    }
}

#[derive(Clone)]
#[cfg(feature = "reqwest")]
pub struct IpApiClient {
    client: reqwest::Client,
    base_url: String,
    access_key: Option<String>,
}
#[cfg(feature = "reqwest")]
extern crate reqwest;
#[cfg(feature = "reqwest")]
impl IpApiClient {
    pub fn new_from_env() -> Self {
        Self::new(if let Ok(x) = std::env::var("IPAPI_ACCESS_KEY") {
            Some(x)
        } else {
            None
        })
    }
    pub fn new(access_key: Option<String>) -> Self {
        Self {
            access_key,
            client: reqwest::Client::new(),
            base_url: "https://api.ipapi.com/api".into(),
        }
    }

    pub async fn lookup(&self, ip: String) -> Result<IpInfo, reqwest::Error> {
        let mut params = vec![];
        if let Some(x) = &self.access_key {
            params.push(("access_key", x.clone()));
        }
        let mut url = reqwest::Url::parse_with_params(&self.base_url, params).unwrap();
        url.path_segments_mut().unwrap().push(ip.as_str());
        Ok(self.client.get(url).send().await?.json().await?)
    }
}
