use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("Compilation error in Rust code: {filepath} ")]
    InvalidRustCode{filepath: String},
    #[error("Unable to read file:  {filepath} ")]
    InvalidSourceFile{filepath: String},
    #[error("Parsing failed for TopState : {top_state_name}")]
    InvalidTopStateParsing{top_state_name: String, #[source] source: Box<dyn std::error::Error>},
    #[error("Parsing failed for State : {state_name}, caused by {source}")]
    InvalidStateParsing{state_name: String, #[source] source: Box<dyn std::error::Error>},
    #[error("invalid state tag")]
    InvalidStateTag,
    #[error("invalid state machine name")]
    InvalidStateMachineName
}
