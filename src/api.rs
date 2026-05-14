use crate::config::ApiConfig;
use crate::post::{ApiResponse, Post};

pub struct ApiClient {
    pub client: reqwest::blocking::Client,
    pub api_config: ApiConfig,
}

impl ApiClient {
    pub fn new(api_config: ApiConfig) -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent("lemmy-tray")
                .build()
                .unwrap(),
            api_config,
        }
    }
    pub fn get_post(&self) -> Post {
        let mut response: ApiResponse = self
            .client
            .get(self.api_config.build_url())
            .send()
            .and_then(|r| r.json())
            .unwrap();

        (&response.posts.remove(0)).into()
    }
}
