use crate::config::ApiConfig;
use crate::errors::Errors;
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

    pub fn get_post(&self, short_title_len: usize) -> Result<Post, Errors> {
        let mut response: ApiResponse = self
            .client
            .get(self.api_config.build_url())
            .call()
            .map_err(|e| Errors::GetPostCall(e.to_string()))?
            .body_mut()
            .read_json()
            .map_err(|e| Errors::GetPostRead(e.to_string()))?;

        if response.posts.is_empty() {
            return Err(Errors::GetPostEmptyResponse);
        }
        Ok(Post::from_post_view(
            response.posts.remove(0),
            short_title_len,
            &self.api_config.instance,
        ))
    }
}
