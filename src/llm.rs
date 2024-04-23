use std::future::Future;
pub mod open_ai;

const SYSTEM_PROMPT: &str = "fdas" ;

pub trait LLM {

    fn completion(&self, system_prompt: &str, user_prompt: &str) -> impl Future<Output = String>;

    fn stream_completion(&self, system_prompt: &str, user_prompt: &str);

    fn stream_completion_cmd(&self, system_prompt: &str, user_prompt: &str) -> impl Future<Output = ()>;

}