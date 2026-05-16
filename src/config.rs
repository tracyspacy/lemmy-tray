const DEFAULT_INSTANCE: &str = "https://lemmy.ml";
//const DEFAULT_INSTANCE: &str = "http://localhost:8536";
const DEFAULT_LIMIT: u8 = 1;
pub const DEFAULT_TITLE_LEN: usize = 27;
pub const DEFAULT_REFRESH_TICK: u64 = 60;

#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum ListingType {
    All,
    Local,
}

#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum SortType {
    New,
    Active,
    Hot,
    TopDay, //later add more
}

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
    pub fn build_url(&self) -> String {
        format!(
            "{}/api/v3/post/list?type_={}&sort={}&limit={}",
            self.instance, self.listing_type, self.sort_type, DEFAULT_LIMIT
        )
    }
}
