use core::fmt;
use std::fs::File;
use std::io::Read;
use crate::{errors::Error, share::StateMachine};
use proc_macro2::TokenStream;
use syn::{parse::Parse, PathArguments, GenericArgument, Type, ImplItem};

use crate::share::{State, self};

pub struct Parser{
    file_path : String    
}
#[derive(Debug)]
pub enum TraitImplHeaderInfo<'a>{
    State{state_tag : String, state_machine_name: String, impl_body: &'a Vec<ImplItem>},
    TopState{state_machine_name: String, impl_body: &'a Vec<ImplItem>},
    Other
}

impl Parser{

    pub fn new(file_path: &String) -> Parser{
        Parser{file_path: file_path.clone()}
    }
    
    fn get_state_tag(path_argument: &PathArguments) -> Result<String, Error>{
        if let PathArguments::AngleBracketed(ab) = path_argument{
            if let GenericArgument::Type(generic_arg_0) = &ab.args[0]{
                if let Type::Path(path) = generic_arg_0{
                    let ident = &path.path.segments[0].ident;
                    return Ok(ident.to_string())
                } 
             }
        }
        return Err(Error::InvalidStateTag);
     }

    fn get_state_type(self_ty: &Type) -> Result<String, Error>{
            if let Type::Path(path) = self_ty{
                    let ident = &path.path.segments[0].ident;
                    return Ok(ident.to_string())
                }
            else { Err(Error::InvalidStateMachineName)}
    }

    pub fn get_trait_impl_type(trait_impl :  &syn::ItemImpl) -> Result<TraitImplHeaderInfo, Error>{
        let trait_impl_header_info : TraitImplHeaderInfo;
        if let Some(trait_) = &trait_impl.trait_{
            let path = &trait_.1;
            let segment = &path.segments[0];
            let trait_impl_ident = &segment.ident;
            if trait_impl_ident.to_string() == "State"{
                        trait_impl_header_info = TraitImplHeaderInfo::State {
                        state_tag: Self::get_state_tag(&segment.arguments)?,
                        state_machine_name : Self::get_state_type(&trait_impl.self_ty)?,
                        impl_body: &trait_impl.items 
                };
                    
            }
            else if trait_impl_ident.to_string() == "TopState"{
                trait_impl_header_info = TraitImplHeaderInfo::TopState {
                state_machine_name: Self::get_state_type(&trait_impl.self_ty)?,
                impl_body: &trait_impl.items 
                }
            }
            else{
                trait_impl_header_info = TraitImplHeaderInfo::Other;
            }
            return Ok(trait_impl_header_info);
        }
        panic!("Should not happen")
    }
    
    fn check_state_machine_ownership(label: String, state_machine_model :&mut StateMachine) -> Result<(), Error>{
        if label == state_machine_model.label{
            return Ok(());
        }
        else{
            if state_machine_model.label.is_empty(){
               state_machine_model.label = label;
                return Ok(());
            }
            else{
                return Err(Error::ConcurrentStateMachineImpl { expected_state_machine_name: state_machine_model.label.clone(), found_state_machine_name: label });
            }
        }
    }
    
    pub fn fill_top_state_model(state_trait_impl_body : &Vec<ImplItem>,state_machine_model: &mut share::StateMachine){
        
    }

    pub fn fill_state_model(state_tag : String, top_state_trait_impl_body : &Vec<ImplItem>, state_machine_model: &mut share::StateMachine){

    }

    pub fn parse(& self ) -> Result<Box<State>, Error>{
        // Extract file content to string
        let mut file = File::open(&self.file_path).unwrap();
        let mut content = String::new(); 
        file.read_to_string(&mut content).map_err(|_err| Error::InvalidSourceFile { filepath: self.file_path.clone()})?;
        
        // Convert string to AST
        let ast = syn::parse_file(&content).map_err(|_err| Error::InvalidRustCode  { filepath: self.file_path.clone()})?;
        
        let mut state_machine_model = share::StateMachine::new();

        for item in ast.items.iter(){
            if let syn::Item::Impl(trait_impl) = item {
               let trait_impl_info = Self::get_trait_impl_type(&trait_impl)?;
                match trait_impl_info{
                    TraitImplHeaderInfo::State { state_tag, state_machine_name, impl_body } => {
                        Self::check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        Self::fill_state_model(state_tag,impl_body, &mut state_machine_model);
                    }
                    TraitImplHeaderInfo::TopState {state_machine_name, impl_body} => {
                        Self::check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        Self::fill_top_state_model(impl_body, &mut state_machine_model);
                    }
                    TraitImplHeaderInfo::Other => ()
                }
                //println!("{:?}", trait_impl_info);
            }
        }
        unimplemented!();
    }

}

