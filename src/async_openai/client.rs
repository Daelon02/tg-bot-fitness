use crate::consts::OPEN_AI_MODEL;
use crate::errors::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub struct OpenAiClient {
    pub client: Client<OpenAIConfig>,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::default().with_api_key(api_key);
        let client = Client::with_config(config);
        Self { client }
    }

    pub async fn send_message(&self, text: &str) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(OPEN_AI_MODEL)
            .messages([ChatCompletionRequestSystemMessageArgs::default()
                .content(text)
                .build()?
                .into()])
            .build()?;

        // Call API
        let response = self
            .client
            .chat() // Get the API "group" (completions, images, etc.) from the client
            .create(request) // Make the API call in that "group"
            .await?;

        Ok(response.choices[0]
            .message
            .content
            .clone()
            .expect("Can't get content"))
    }
}
