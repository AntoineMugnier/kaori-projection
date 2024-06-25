use crate::{errors::Error, model, parser, parser::init};
use syn::{ImplItem,ItemImpl, spanned::Spanned};
mod handle;

pub fn fill_state_model(trait_impl : &ItemImpl, state_tag : String, state_machine_model: &mut model::StateMachine) -> Result<(), Error>{
        parser::try_insert_state_into_model(state_machine_model, state_tag.clone()); 
        let state_trait_impl_body =  &trait_impl.items;
        let mut init = None;
        let mut entry = None;
        let mut exit = None;
        let mut evt_handler = None;

        for item in state_trait_impl_body.iter(){
            if let ImplItem::Fn(fn_) = item{
                match fn_.sig.ident.to_string().as_str() {
                    "init" =>{
                       let (init_target, action) = init::parse_init_fn(fn_)?;
                       let mut new_init = model::Init::new();
                       new_init.action = action;
                       new_init.target = init_target;
                       init = Some(new_init);
                    }
                    "entry" =>{
                        let action = parser::parse_entry_or_exit_fn(fn_);
                        entry = Some(model::Entry{action});
                    }
                    "exit" =>{
                        let action = parser::parse_entry_or_exit_fn(fn_);
                        exit = Some(model::Exit{action});
                    }
                    "handle" =>{
                       evt_handler = Some(handle::parse_handle(fn_)?);
                    }
                    _ => ()
                }
            }
        }
        
        let state = state_machine_model.states.get_mut(&state_tag).unwrap();
        
        state.init = init; 
        state.entry = entry;
        state.exit = exit;

        if let Some(evt_handler) = evt_handler{
            state.evt_handler = evt_handler;
        }
        else{
            let line_col = trait_impl.span().start();
            return Err(Error::MissingEvtHandler {line: line_col.line, col: line_col.column})
        }

        return Ok(())
    }


