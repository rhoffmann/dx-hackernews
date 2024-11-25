use dioxus::prelude::*;

use crate::{CommentData, StoryPageData};

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(Box<StoryPageData>),
}

#[component]
pub fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();
    match preview_state() {
        PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(story) => {
            rsx! {
                div {
                    class: "p-4",
                    div {
                        class: "font-xl",
                        a {
                            href: story.item.url,
                            "{story.item.title}"
                        }
                    }
                    div {
                        dangerous_inner_html: story.item.text
                    }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Comment(comment: CommentData) -> Element {
    rsx! {
        div {
            class: "p-4",
            div {
                class: "text-gray-600",
                "by {comment.by}"
            }
            div {
                dangerous_inner_html: "{comment.text}"
            }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}
