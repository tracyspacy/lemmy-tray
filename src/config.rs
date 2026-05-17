use crate::errors::Errors;
use serde::Deserialize;
const DEFAULT_INSTANCE: &str = "lemmy.ml";
//const DEFAULT_INSTANCE: &str = "http://localhost:8536";
const DEFAULT_LIMIT: u8 = 1;
pub const DEFAULT_TITLE_LEN: usize = 27;
pub const DEFAULT_REFRESH_TICK: u64 = 60;

#[derive(
    strum_macros::EnumIter,
    strum_macros::EnumString,
    strum_macros::Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Deserialize,
    Debug,
)]
pub enum ListingType {
    All,
    Local,
}

#[derive(
    strum_macros::EnumIter,
    strum_macros::EnumString,
    strum_macros::Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Deserialize,
    Debug,
)]
pub enum SortType {
    New,
    Active,
    Hot,
    TopDay, //do we need this?
    TopHour,
    NewComments,
    Scaled,
    Controversial, //later add more?
}

// todo - specify config path for binaries
fn parse_config() -> Result<ApiConfig, Errors> {
    let config_str =
        std::fs::read_to_string("config.toml").map_err(|_| Errors::FailedToReadConfig)?;
    let config: ApiConfig = toml::from_str(&config_str).map_err(|_| Errors::FailedToParseConfig)?;
    Ok(config)
}

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    pub instance: String,
    pub listing_type: ListingType,
    pub sort_type: SortType,
}
impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            instance: DEFAULT_INSTANCE.to_string(),
            listing_type: ListingType::All,
            sort_type: SortType::Active,
        }
    }
}

impl ApiConfig {
    pub fn load_config() -> Result<Self, Errors> {
        parse_config()
    }

    pub fn build_url(&self) -> String {
        format!(
            "https://{}/api/v3/post/list?type_={}&sort={}&limit={}",
            self.instance, self.listing_type, self.sort_type, DEFAULT_LIMIT
        )
    }
}
