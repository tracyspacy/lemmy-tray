use crate::config::ApiConfig;
use crate::post::{ApiResponse, Post};

pub struct ApiClient {
    pub client: ureq::Agent,
    pub api_config: ApiConfig,
}

impl ApiClient {
    pub fn new(api_config: ApiConfig) -> Self {
        Self {
            client: ureq::Agent::new_with_defaults(),
            api_config,
        }
    }
    //Todo:add error handling
    pub fn get_post(&self) -> Post {
        let mut response: ApiResponse = self
            .client
            .get(self.api_config.build_url())
            .call()
            .unwrap()
            .body_mut()
            .read_json()
            .unwrap();

        (&response.posts.remove(0)).into()
    }
}
