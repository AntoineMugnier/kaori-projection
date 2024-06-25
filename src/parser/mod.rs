use std::fs::File;
use std::io::Read;
use crate::{errors::Error, model::StateMachine};
use syn::{PathArguments, GenericArgument, Type, ImplItemFn, spanned::Spanned};
pub(crate) mod top_state;
pub(crate) mod init;
pub(crate) mod state;

use crate::model::{State, self};

pub struct Parser{
    file_path : String    
}

#[derive(Debug)]
pub enum TraitImplSignatureInfo{
    State{state_tag : String, state_machine_name: String},
    TopState{state_machine_name: String},
    Other
}

impl Parser{

    pub fn new(file_path: &String) -> Parser{
        Parser{file_path: file_path.clone()}
    }

    pub fn parse(& self ) -> Result<Box<State>, Error>{
        // Extract file content to string
        let mut file = File::open(&self.file_path).unwrap();
        let mut content = String::new(); 
        file.read_to_string(&mut content).map_err(|_err| Error::InvalidSourceFile { filepath: self.file_path.clone()})?;
        
        // Convert string to AST
        let ast = syn::parse_file(&content).map_err(|_err| Error::InvalidRustCode  { filepath: self.file_path.clone()})?;
        
        let mut state_machine_model = model::StateMachine::new();

        for item in ast.items.iter(){
            if let syn::Item::Impl(trait_impl) = item {
               let trait_impl_info = top_state::get_trait_impl_type(&trait_impl)?;
                match trait_impl_info{
                    TraitImplSignatureInfo::State { state_tag, state_machine_name} => {
                        check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        state::fill_state_model(trait_impl, state_tag, &mut state_machine_model)?;
                    }
                    TraitImplSignatureInfo::TopState {state_machine_name} => {
                        check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        top_state::fill_top_state_model(trait_impl, &mut state_machine_model)?;
                    }
                    TraitImplSignatureInfo::Other => ()
                }
                //println!("{:?}", trait_impl_info);
            }
        }
        println!("{:#?}", state_machine_model);
        unimplemented!();
    }
}

    fn get_state_tag(path_argument: &PathArguments) -> Result<String, Error>{
        if let PathArguments::AngleBracketed(ab) = path_argument{
            if let GenericArgument::Type(generic_arg_0) = &ab.args[0]{
                if let Type::Path(path) = generic_arg_0{
                    let ident = &path.path.segments.last().unwrap().ident;
                    return Ok(ident.to_string())
                } 
             }
        }
        let line_col = path_argument.span().start();
        return Err(Error::InvalidStateTag{line: line_col.line, col: line_col.column});
     }


    
    fn check_state_machine_ownership(label: String, state_machine_model :&mut StateMachine) -> Result<(), Error>{
        if label == state_machine_model.name{
            return Ok(());
        }
        else{
            if state_machine_model.name.is_empty(){
               state_machine_model.name = label;
                return Ok(());
            }
            else{
                return Err(Error::ConcurrentStateMachineImpl { expected_state_machine_name: state_machine_model.name.clone(), found_state_machine_name: label });
            }
        }
    }

    
    pub fn try_insert_state_into_model(state_machine_model: &mut model::StateMachine, state_tag: String){
        if !state_machine_model.states.contains_key(&state_tag){
            let mut state = model::State::new();
            state.name = state_tag.clone();
            state_machine_model.states.insert(state_tag.clone(), state);
        }
    }
   


    // Same handling for parsing both functions
    pub fn parse_entry_or_exit_fn(fn_: &ImplItemFn) -> Option<String>{
        let stmts = &fn_.block.stmts;
        let mut action = None;
        if let Some(stmt) = stmts.last(){
                action = stmt.span().source_text();
        }
        action
    }
    






