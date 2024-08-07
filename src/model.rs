use std::collections::HashMap;

#[derive(Debug, PartialEq)]
 pub enum Next{
    Condition(Condition),
    Target(TransitionTarget),
    Handled(),
    Unterminated()
}


#[derive(Debug, PartialEq)]
pub struct Init{
    pub action: Option<String>,
    pub target: String
}

#[derive(Debug, PartialEq)]
pub struct Entry{
    pub action: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct Exit{
    pub action: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct Condition{
    pub branches: Vec<ConditionalBranch>
}

#[derive(Debug, PartialEq)]
pub struct ConditionalBranch{
    pub guard: String, 
    pub action: Option<String>, 
    pub next: Next 
}

#[derive(Debug, PartialEq)]
pub struct TransitionTarget{
    pub state_name: String
}


#[derive(Debug, PartialEq)]
pub struct EvtCatcher{
    pub evt_type_name: String,
    pub evt_variant_name : String,
    pub action: Option<String>,
    pub next: Next 
}

#[derive(Debug, PartialEq)]
pub struct EvtHandler{
    pub evt_catchers: Vec<EvtCatcher> 
}

#[derive(Debug, PartialEq)]
pub struct State{
    pub name: String,
    pub entry: Option<Entry>,
    pub exit: Option<Exit>,
    pub init: Option<Init>,
    pub evt_handler: EvtHandler
}

#[derive(Debug, PartialEq)]
pub struct TopState{
    pub evt_type_alias: Option<String>,
    pub init: Init,
}

#[derive(Debug, PartialEq)]
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

impl EvtHandler{
    pub fn new() -> EvtHandler{
        EvtHandler{
            evt_catchers: Vec::new()
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
            evt_handler: EvtHandler::new() 
        }
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
