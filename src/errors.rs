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
    InvalidStateMachineName,
    #[error("State machine {expected_state_machine_name} and {found_state_machine_name} cannot be implemented in the same file")]
    ConcurrentStateMachineImpl{expected_state_machine_name: String, found_state_machine_name: String},
    #[error("Ill-formed state machine, does your code compile?")]
    IllFormedStateMachine,
    #[error("Missing Topstate::Evt type alias definition")]
    MissingEvtTypeDef,
    #[error("Missing Topstate::init function definition")]
    MissingTopStateInitDef,
    #[error("Invalid Topstate::Evt type alias definition")]
    InvalidEvtTypeDef,
    #[error("Missing call to init_transition() macro in `TopState::Init()` function")]
    MissingTopStateInitTranCall,
}
