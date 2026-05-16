use crate::config::DEFAULT_TITLE_LEN;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub posts: Vec<PostView>,
}

#[derive(Deserialize, Debug)]
pub struct PostView {
    post: PostData,
    community: CommunityData,
    counts: Counts,
}

#[derive(Deserialize, Debug)]
struct PostData {
    name: String,
    ap_id: String,
}

#[derive(Deserialize, Debug)]
struct CommunityData {
    name: String,
    actor_id: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Counts {
    // score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub comments: i64,
}

#[derive(Debug)]
pub struct Post {
    pub full_title: String,
    pub short_title: String,
    pub community: String,
    pub url: Option<String>,
    pub counts: Counts,
}

impl From<PostView> for Post {
    fn from(post_view: PostView) -> Self {
        let short_title = Self::prepare_short_title(&post_view.post.name);
        Self {
            full_title: post_view.post.name,
            short_title,
            community: Self::prepare_community(
                &post_view.community.name,
                &post_view.community.actor_id,
            ),
            url: Some(post_view.post.ap_id),
            counts: Counts {
                upvotes: post_view.counts.upvotes,
                downvotes: post_view.counts.downvotes,
                comments: post_view.counts.comments,
            },
        }
    }
}

impl Post {
    pub fn placeholder_post(msg: &str) -> Self {
        Self {
            full_title: msg.to_string(),
            short_title: msg.to_string(),
            community: String::new(),
            url: None,
            counts: Counts {
                upvotes: 0,
                downvotes: 0,
                comments: 0,
            },
        }
    }
    //helpers
    //maybe fill with spaces if short, so every title have same len
    fn prepare_short_title(full_title: &str) -> String {
        let mut chars = full_title.chars().peekable();
        let mut short_title = String::with_capacity(DEFAULT_TITLE_LEN);
        for _ in 0..DEFAULT_TITLE_LEN {
            match chars.next() {
                Some(char) => short_title.push(char),
                None => return short_title,
            }
        }
        if chars.peek().is_some() {
            for _ in 0..3 {
                short_title.pop();
            }
            short_title.push_str("...");
        }
        short_title
    }

    fn prepare_community(community_name: &str, instance: &str) -> String {
        format!(
            "!{}@{}",
            community_name,
            instance.split('/').nth(2).unwrap_or("unknown instance")
        )
    }
}
