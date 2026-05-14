const DEFAULT_INSTANCE: &str = "https://lemmy.ml";
//const DEFAULT_INSTANCE: &str = "http://localhost:8536";
const DEFAULT_LIMIT: u8 = 1;
pub const DEFAULT_TITLE_LEN: usize = 27;
pub const DEFAULT_REFRESH_TICK: u64 = 60;
pub enum ListingType {
    All,
    Local,
}
impl ListingType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "All",
            Self::Local => "Local",
        }
    }
}
pub enum SortType {
    New,
    Active,
    Hot,
    TopDay, //later add more
}

impl SortType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "New",
            Self::Active => "Active",
            Self::Hot => "Hot",
            Self::TopDay => "TopDay",
        }
    }
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
            self.instance,
            self.listing_type.as_str(),
            self.sort_type.as_str(),
            DEFAULT_LIMIT
        )
    }
}
