use std::collections::HashMap;
use std::rc;
use std::marker;

 pub enum Next<'a>{
    ConditionalBranch(Box<ConditionalBranch<'a>>),
    Target(rc::Rc<State<'a>>)
}

pub struct Entry{
    label: String
}
pub struct Exit{
    label: String
}
pub struct Init<'a>{
    label: String,
    target: rc::Rc<State<'a>>
}

pub struct ConditionalBranch<'a>{
    label: String, 
    next: Vec<Next<'a>> 
}

pub struct EvtHandler<'a>{
    label: String,
    next: Vec<Next<'a>> 
}

pub struct State<'a>{
    phantom_data: marker::PhantomData<&'a u8>,
    label: String,
    entry: Option<Entry>,
    exit: Option<Exit>,
    init: Option<Init<'a>>,
    evt_handlers: Vec<Box<EvtHandler<'a>>>
}

pub struct TopState<'a>{
    label: String,
    init: Option<Init<'a>>,
}

pub struct StateMachine<'a>{
    pub label: String,
    top_state: TopState<'a>,
    states: HashMap<String, State<'a>>
}

impl <'a> StateMachine<'a>{
    pub fn new() -> StateMachine<'a>{
        StateMachine{
            label: String::new(),
            top_state: TopState { label: String::new(), init: None },
            states: HashMap::new()
        }
    }
}
