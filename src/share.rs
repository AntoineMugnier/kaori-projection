use std::rc;
use std::marker;

enum Next<'a>{
    ConditionalBranch(Box<ConditionalBranch<'a>>),
    Target(rc::Rc<State<'a>>)
}

struct Entry{
    label: String
}
struct Exit{
    label: String
}
struct Init<'a>{
    label: String,
    target: rc::Rc<State<'a>>
}

struct ConditionalBranch<'a>{
    label: String, 
    next: Vec<Next<'a>> 
}

struct EvtHandler<'a>{
    label: String,
    next: Vec<Next<'a>> 
}

struct State<'a>{
    phantom_data: marker::PhantomData<&'a u8>,
    label: String,
    entry: Option<Entry>,
    exit: Option<Exit>,
    init: Option<Init<'a>>,
    evt_handlers: Vec<Box<EvtHandler<'a>>>
}

