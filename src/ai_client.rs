use bevy::log::info;
use derive_builder::Builder;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::common::GPT3_5_TURBO;

#[derive(Builder, Clone)]
pub struct ClientConfig {
    #[builder(default = "\"https://api.openai.com/v1\".to_owned()")]
    api_endpoint: String,
    #[builder(default = "std::env::var(\"OPENAI_API_KEY\").expect(\"OPENAI_API_KEY not set\")")]
    api_key: String,
    #[builder(default = "None")]
    proxy: Option<String>,
}

pub(crate) fn send_to_ai_internal(prompt: String, client_config: ClientConfig) -> String {
    let mut client = Client::new(client_config.api_key);
    client.api_endpoint = client_config.api_endpoint;
    client.proxy = client_config.proxy;

    info!("prompt:{}", prompt);
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(prompt),
            name: None,
        }],
    );
    let response = client.chat_completion(req).expect("Failed to send request");
    let res_str = response.choices[0]
        .message
        .content
        .clone()
        .expect("Failed to get response");
    info!("response:{}", res_str);
    res_str
}