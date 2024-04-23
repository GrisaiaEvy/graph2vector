use std::io::{stdout, Write};
use async_openai::Client;
use async_openai::config::{Config, OpenAIConfig};
use async_openai::types::{ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateCompletionRequest};
use crate::llm::LLM;
use futures::StreamExt;

pub struct OpenAi {
    api_base: String,
    api_key: String,
    model: String,
    temperature: f32

}

impl OpenAi {
    pub fn new(url: String, key: String, model: String, temperature: f32) -> OpenAi {
        OpenAi { api_base: url, api_key: key, model, temperature }
    }

    pub fn request(&self, system_prompt: &str, user_prompt: &str) -> CreateChatCompletionRequest {
        CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .max_tokens(512u16)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build().expect("build system prompt failed")
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build().expect("build user prompt failed")
                    .into()])
            .build().expect("build llm request failed")
    }

    pub fn client(&self) -> Client<OpenAIConfig> {
        Client::with_config(OpenAIConfig::new()
            .with_api_base(self.api_base.as_str())
            .with_api_key(self.api_key.as_str()))
    }

}

impl LLM for OpenAi {

    async fn completion(&self, system_prompt: &str, user_prompt: &str) -> String {
        let client = self.client();
        let request = self.request(system_prompt, user_prompt);

        let response = client.chat().create(request).await.expect("fetch llm response error");

        let mut result = String::new();

        for choice in response.choices {
            println!(
                "{}: Role: {}  Content: {:?}",
                choice.index, choice.message.role, choice.message.content
            );
            result.push_str(choice.message.content.expect("fetch ").as_str())
        }
        result
    }

    fn stream_completion(&self, system_prompt: &str, user_prompt: &str) {
        todo!()
    }

    async fn stream_completion_cmd(&self, system_prompt: &str, user_prompt: &str) {
        let client = self.client();
        let request = self.request(system_prompt, user_prompt);

        let mut stream = client.chat().create_stream(request).await.expect("fetch llm stream error");

        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                        }
                    });
                }
                Err(err) => {
                    writeln!(lock, "error: {err}").unwrap();
                }
            }
            stdout().flush().expect("flush error");
        }
    }
}