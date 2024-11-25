use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,

    #[serde(default)]
    pub comments: Vec<CommentData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentData {
    pub id: i64,

    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub text: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,

    #[serde(default)]
    pub kids: Vec<i64>,

    #[serde(default)]
    pub sub_comments: Vec<CommentData>,

    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,

    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub score: i64,

    #[serde(default)]
    pub descendants: i64,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,

    #[serde(default)]
    pub kids: Vec<i64>,

    pub r#type: String,
}

#[component]
pub fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id:0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants:0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string()
            }
        }
    }
}

#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();

    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });

    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );

    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            class: "p-4 relative",
            div {
                class: "text-xl text-blue-500",
                a { href: url, "{title}"}
                a {
                    class: "text-gray-600",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    " ({ hostname})"
                }
            }
            div {
                class: "flex flex-row text-gray-600",
                div { "{score}" }
                div { class: "pl-2", "by {by}" }
                div { class: "pl-2", "{time}" }
                div { class: "pl-2", "{comments}"}
            }
        }
    }
}
