use std::collections::HashMap;

#[derive(Debug)]
 pub enum Next{
    ConditionalBranch(ConditionalBranch),
    Target(TransitionTarget)
}

#[derive(Debug)]
pub struct Init{
    pub action: Option<String>,
    pub target: String
}

#[derive(Debug)]
pub struct Entry{
    pub action: Option<String>
}

#[derive(Debug)]
pub struct Exit{
    pub action: Option<String>
}

#[derive(Debug)]
pub struct ConditionalBranch{
    pub guard: String, 
    pub action: String, 
    pub next: Vec<Next> 
}

#[derive(Debug)]
pub struct TransitionTarget{
    pub state_name: String
}


#[derive(Debug)]
pub struct EvtHandler{
    pub evt_name : String,
    pub action: String,
    pub next: Vec<Next> 
}

#[derive(Debug)]
pub struct State{
    pub name: String,
    pub entry: Option<Entry>,
    pub exit: Option<Exit>,
    pub init: Option<Init>,
    pub evt_handlers: Vec<EvtHandler>
}

#[derive(Debug)]
pub struct TopState{
    pub evt_type_alias: Option<String>,
    pub init: Init,
}

#[derive(Debug)]
pub struct StateMachine{
    pub name: String,
    pub top_state: TopState,
    pub states: HashMap<String, State>
}

impl  Init{
    pub fn new() -> Init{
        Init {
            action: None,
            target: String::new()
        }
    }
}

impl  State{
    pub fn new() -> State{
        State {
            name: String::new(),
            entry: None,
            exit: None,
            init: None,
            evt_handlers: Vec::new()}
    }
}
impl  StateMachine{
    pub fn new() -> StateMachine{
        StateMachine{
            name: String::new(),
            top_state: TopState { evt_type_alias: None, init: Init::new()},
            states: HashMap::new()
        }
    }
}
