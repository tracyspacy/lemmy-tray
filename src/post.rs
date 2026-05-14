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
    pub url: String,
    pub counts: Counts,
}

impl From<&PostView> for Post {
    fn from(post_view: &PostView) -> Self {
        Self {
            full_title: post_view.post.name.clone(),
            short_title: Self::prepare_short_title(&post_view.post.name),
            community: Self::prepare_community(
                &post_view.community.name,
                &post_view.community.actor_id,
            ),
            url: post_view.post.ap_id.clone(),
            counts: Counts {
                upvotes: post_view.counts.upvotes,
                downvotes: post_view.counts.downvotes,
                comments: post_view.counts.comments,
            },
        }
    }
}

impl Post {
    //helpers
    fn prepare_short_title(full_title: &str) -> String {
        if full_title.len() > DEFAULT_TITLE_LEN {
            full_title
                .chars()
                .take(DEFAULT_TITLE_LEN)
                .collect::<String>()
                + "..."
        } else {
            //maybe fill with sapces if short, so every title have same len
            full_title.to_string()
        }
    }

    fn prepare_community(community_name: &str, instance: &str) -> String {
        format!(
            "!{}@{}",
            community_name,
            instance.split('/').nth(2).unwrap_or("unknown instance")
        )
    }
}
