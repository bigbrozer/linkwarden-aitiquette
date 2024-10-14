use openai_api_rs::v1::chat_completion::{ChatCompletionMessage, Content, MessageRole};

use crate::api::Link;

pub fn build_summary() -> ChatCompletionMessage {
    return ChatCompletionMessage {
        role: MessageRole::system,
        content: Content::Text("You are a summary bot. You must answer with a very short summary (5-10 lines). Do not introduce what you will do.".to_string()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };
}

pub fn build_tagging() -> ChatCompletionMessage {
    return ChatCompletionMessage {
        role: MessageRole::system,
        content: Content::Text(r#"
You are a bot in a read-it-later app and your responsibility is to help with automatic tagging.
Please analyze the text and suggest relevant tags that describe its key themes, topics, and main ideas. The rules are:
- Aim for a variety of tags, including broad categories, specific keywords, and potential sub-genres.
- The tags language must be in French.
- If it's a famous website you may also include a tag for the website. If the tag is not generic enough, don't include it.
- The content can include text for cookie consent and privacy policy, ignore those while tagging.
- Aim for 3-5 tags.
- If there are no good tags, leave the array empty.
You must respond in single-line JSON with the key "tags" and the value is an array of string tags."#.to_string()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };
}

pub fn for_link(link: &Link) -> ChatCompletionMessage {
    return ChatCompletionMessage {
        role: MessageRole::user,
        content: Content::Text(format!(
            "{}\n{}\n{}",
            link.name, link.url, link.text_content
        )),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };
}
