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

