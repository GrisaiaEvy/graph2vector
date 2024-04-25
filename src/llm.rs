use std::error::Error;
use std::future::Future;
pub mod open_ai;

const SYSTEM_PROMPT: &str = "fdas" ;

pub trait LLM {

    fn completion(&self, system_prompt: &str, user_prompt: &str) -> impl Future<Output = Result<String, Box<dyn Error>>>;

    fn stream_completion(&self, system_prompt: &str, user_prompt: &str);

    fn stream_completion_cmd(&self, system_prompt: &str, user_prompt: &str) -> impl Future<Output = ()>;

}