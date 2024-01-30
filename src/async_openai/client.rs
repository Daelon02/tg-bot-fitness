use crate::consts::OPEN_AI_MODEL;
use crate::errors::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateCompletionRequestArgs;
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
        let request = CreateCompletionRequestArgs::default()
            .model(OPEN_AI_MODEL)
            .prompt(text)
            .max_tokens(40_u16)
            .build()?;

        // Call API
        let response = self
            .client
            .completions() // Get the API "group" (completions, images, etc.) from the client
            .create(request) // Make the API call in that "group"
            .await?;

        Ok(response.choices[0].text.clone())
    }
}
