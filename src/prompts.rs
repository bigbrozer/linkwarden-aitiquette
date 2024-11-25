use openai_api_rs::v1::chat_completion::{ChatCompletionMessage, Content, MessageRole};

use crate::models::Link;

pub fn build_summary() -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: MessageRole::system,
        content: Content::Text(
            "You receive complete article content as input. You must condense it to 1-3 sentences maximum. Only return the answer."
                .to_string(),
        ),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
}

pub fn build_tagging() -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: MessageRole::system,
        content: Content::Text(r#"
You are a bot in a read-it-later app and your responsibility is to help with automatic tagging.
Please analyze the text and suggest relevant tags that describe its key themes, topics, and main ideas. Follow strictly the following rules:
- Aim for a variety of tags, including broad categories, specific keywords, and potential sub-genres.
- Write tags in french.
- If it's a famous website you may also include a tag for the website. If the tag is not generic enough, don't include it.
- The content can include text for cookie consent and privacy policy, ignore those while tagging.
- Aim for 3-5 short and concise tags.
- If there are no good tags, leave the array empty.
You must respond as a single line JSON string (do not add extra lines) which is an array of string tags."#.to_string()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
}

pub fn for_link(link: &Link) -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: MessageRole::user,
        content: Content::Text(format!(
            "{}\n{}\n{}",
            link.name,
            link.url,
            link.text_content.as_deref().unwrap_or_default()
        )),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
}

pub fn for_link_with_summary(link: &Link, summary: String) -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: MessageRole::user,
        content: Content::Text(format!("{}\n{}\n{}", link.name, link.url, summary)),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
}
