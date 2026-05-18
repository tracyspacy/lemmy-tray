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
    id: u64,
    name: String,
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
    pub comments: u64,
}

#[derive(Debug)]
pub struct Post {
    pub full_title: String,
    pub short_title: String,
    pub community: String,
    pub url: Option<String>,
    pub counts: Counts,
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
    pub fn from_post_view(post_view: PostView, short_title_len: usize, instance_url: &str) -> Self {
        let short_title = Self::prepare_short_title(&post_view.post.name, short_title_len);
        Self {
            full_title: post_view.post.name,
            short_title,
            community: Self::prepare_community(
                &post_view.community.name,
                &post_view.community.actor_id,
            ),
            url: Some(Self::prepare_post_url(instance_url, post_view.post.id)),
            counts: Counts {
                upvotes: post_view.counts.upvotes,
                downvotes: post_view.counts.downvotes,
                comments: post_view.counts.comments,
            },
        }
    }

    //maybe fill with spaces if short, so every title have same len
    fn prepare_short_title(full_title: &str, short_title_len: usize) -> String {
        let mut chars = full_title.chars().peekable();
        let mut short_title = String::with_capacity(short_title_len);
        for _ in 0..short_title_len {
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
    fn prepare_post_url(instance_url: &str, post_id: u64) -> String {
        format!("https://{}/post/{}", instance_url, post_id)
    }
}
