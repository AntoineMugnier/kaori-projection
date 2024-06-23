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
    #[error("L:{line} C:{col} Invalid state tag")]
    InvalidStateTag{line : usize, col: usize},
    #[error("L:{line} C:{col} Invalid state machine name")]
    InvalidStateMachineName{line : usize, col: usize},
    #[error("State machine {expected_state_machine_name} and {found_state_machine_name} cannot be implemented in the same file")]
    ConcurrentStateMachineImpl{expected_state_machine_name: String, found_state_machine_name: String},
    #[error("L:{line} C:{col} Missing Topstate::Evt type alias definition in Topstate trait implementation")]
    MissingEvtTypeDef{line : usize, col: usize},
    #[error("L:{line} C:{col} Missing Topstate::init function definition in Topstate trait implementation")]
    MissingInitDef{line : usize, col: usize},
    #[error("L:{line} C:{col} Invalid Topstate::Evt type alias definition")]
    InvalidEvtTypeDef{line : usize, col: usize},
    #[error("L:{line} C:{col} Missing call to init_transition() macro ")]
    MissingInitTranCall{line : usize, col: usize},
    #[error("L:{line} C:{col} Missing definition of handle() function in State trait")]
    MissingEvtHandleFunction{line : usize, col: usize},
    #[error("L:{line} C:{col} Missing match to enum evt in handle() function")]
    MissingEvtHandler{line : usize, col: usize},
    #[error("L:{line} C:{col} Error parsing event variant in match arm")]
    InvalidEvtMatch{line : usize, col: usize},
}
