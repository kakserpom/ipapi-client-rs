use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoipData {
    pub status: Option<String>,
    pub message: Option<String>,
    pub continent: Option<String>,
    #[serde(rename = "continentCode")]
    pub continent_code: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub timezone: Option<String>,
    pub offset: Option<i32>,
    pub currency: Option<String>,
    pub isp: Option<String>,
    pub org: Option<String>,
    pub r#as: Option<String>,
    pub asname: Option<String>,
    pub reverse: Option<String>,
    pub mobile: Option<bool>,
    pub proxy: Option<bool>,
    pub hosting: Option<bool>,
    pub query: Option<String>,
}
use std::fmt;
impl fmt::Display for GeoipData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut address_parts = Vec::new();

        if let Some(ref city) = self.city {
            address_parts.push(city.clone());
        }
        if let Some(ref district) = self.district {
            address_parts.push(district.clone());
        }
        if let Some(ref region) = self.region {
            address_parts.push(region.clone());
        }
        if let Some(ref country) = self.country {
            address_parts.push(country.clone());
        }
        if let Some(ref continent) = self.continent {
            address_parts.push(continent.clone());
        }

        write!(f, "{}", address_parts.join(", "))
    }
}

impl GeoipData {
    pub fn success(&self) -> bool {
        match &self.status {
            Some(x) if x.eq("success") => true,
            _ => false,
        }
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
            base_url: "http://ip-api.com/json".into(),
        }
    }

    pub async fn lookup(
        &self,
        ip: Option<String>,
        lang: Option<String>,
        fields: Option<Vec<String>>,
    ) -> Result<GeoipData, reqwest::Error> {
        let mut params = vec![];
        if let Some(x) = lang {
            params.push(("lang", x));
        }
        if let Some(x) = fields {
            params.push(("fields", x.join(",")));
        }
        if let Some(x) = &self.access_key {
            params.push(("accessKey", x.clone()));
        }
        let mut url = reqwest::Url::parse_with_params(&self.base_url, params).unwrap();
        if let Some(x) = ip {
            url.path_segments_mut().unwrap().push(x.as_str());
        }
        Ok(self.client.get(url).send().await?.json().await?)
    }
}
