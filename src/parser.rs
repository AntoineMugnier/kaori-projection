use std::ops::Deref;
use std::fs::File;
use std::io::Read;
use crate::{errors::Error, share::StateMachine};
use syn::{PathArguments, GenericArgument, Type, ImplItem, ImplItemType, ItemImpl};

use syn::spanned::Spanned;
use crate::share::{State, self};

pub struct Parser{
    file_path : String    
}
#[derive(Debug)]
pub enum TraitImplSignatureInfo<'a>{
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
                    let ident = &path.path.segments.last().unwrap().ident;
                    return Ok(ident.to_string())
                } 
             }
        }
        let line_col = path_argument.span().start();
        return Err(Error::InvalidStateTag{line: line_col.line, col: line_col.column});
     }

    fn get_state_type(self_ty: &Type) -> Result<String, Error>{
            if let Type::Path(path) = self_ty{
                    let ident = &path.path.segments.last().unwrap().ident;
                    return Ok(ident.to_string())
                }
            else {
            let line_col = self_ty.span().start();
            Err(Error::InvalidStateMachineName{line: line_col.line, col: line_col.column})
        }
    }

    pub fn get_trait_impl_type(trait_impl :  &syn::ItemImpl) -> Result<TraitImplSignatureInfo, Error>{
        let trait_impl_signature_info : TraitImplSignatureInfo;
        if let Some(trait_) = &trait_impl.trait_{
            let path = &trait_.1;
            let segment = &path.segments[0];
            let trait_impl_ident = &segment.ident;
            if trait_impl_ident.to_string() == "State"{
                        trait_impl_signature_info = TraitImplSignatureInfo::State {
                        state_tag: Self::get_state_tag(&segment.arguments)?,
                        state_machine_name : Self::get_state_type(&trait_impl.self_ty)?,
                        impl_body: &trait_impl.items 
                };
                    
            }
            else if trait_impl_ident.to_string() == "TopState"{
                trait_impl_signature_info = TraitImplSignatureInfo::TopState {
                state_machine_name: Self::get_state_type(&trait_impl.self_ty)?,
                impl_body: &trait_impl.items 
                }
            }
            else{
                trait_impl_signature_info = TraitImplSignatureInfo::Other;
            }
            return Ok(trait_impl_signature_info);
        }
        panic!("Should not happen")
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
    fn parse_top_state_associated_type(type_: &ImplItemType, state_machine_model: &mut StateMachine) -> Result<(), Error>{
        let type_ident = &type_.ident;
        let type_alias = &type_.ty;
        if type_ident.to_string() == "Evt"{
            if let Type::Path(type_alias) = type_alias{
                let segment = type_alias.path.segments.last();
                if let Some(segment) = segment{
                    state_machine_model.top_state.evt_type_alias = Some(segment.ident.to_string());
                    return Ok(());
                }
            }
        }
        return Err(Error::InvalidEvtTypeDef{line: type_.span().start().line, col: type_.span().start().column});
    } 
    
    pub fn try_insert_state_into_model(state_machine_model: &mut share::StateMachine, state_tag: String){
        if !state_machine_model.states.contains_key(&state_tag){
            let mut state = share::State::new();
            state.name = state_tag.clone();
            state_machine_model.states.insert(state_tag.clone(), state);
        }
    }
   
    fn try_parse_top_init_exprmacro(exprmacro: &syn::ExprMacro, state_machine_model :&mut StateMachine) -> bool{
         if let Some(last_path_segment) = exprmacro.mac.path.segments.last(){
            if last_path_segment.ident.to_string() == "init_transition"{
                let target_state_tag = &exprmacro.mac.tokens.to_string();
                Self::try_insert_state_into_model(state_machine_model, target_state_tag.clone()); 
                state_machine_model.top_state.init_target = Some(target_state_tag.clone());
                return true;
            }
        }
        return false;
    }

    pub fn fill_top_state_model(trait_impl : &ItemImpl,state_machine_model: &mut share::StateMachine) -> Result<(), Error>{
        let state_trait_impl_body =  &trait_impl.items;
        for item in state_trait_impl_body.iter(){
            match item{
                ImplItem::Type(type_) =>{
                     Self::parse_top_state_associated_type(&type_, state_machine_model)?;
                },
                ImplItem::Fn(fn_) =>{
                    if fn_.sig.ident.to_string() == "init"{
                        let stmts = &fn_.block.stmts;
                        for stmt in stmts{
                            if let syn::Stmt::Expr(expr,_ ) = stmt {
                                match expr{
                                   syn::Expr::Macro(exprmacro) => {
                                        if Self::try_parse_top_init_exprmacro(exprmacro, state_machine_model){
                                            break;
                                        } 
                                    }
                                    syn::Expr::Return(expreturn) =>{
                                        let expr = &expreturn.expr;
                                        if let Some(expr) = expr{
                                            if let syn::Expr::Macro(exprmacro) = expr.deref() {
                                                if Self::try_parse_top_init_exprmacro(exprmacro, state_machine_model){
                                                    break;
                                                } 
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            } 
                        }
                        if state_machine_model.top_state.init_target.is_none(){
                            let line_col = fn_.span().start();
                            return Err(Error::MissingTopStateInitTranCall{line: line_col.line, col: line_col.column})
                        }
                    }
                },
                _ =>{}
            }
        }
        
        // Trigger error if all searched items are not found
        if state_machine_model.top_state.init_target.is_none(){
            let line_col = trait_impl.span().start();
            Err(Error::MissingTopStateInitDef{line: line_col.line, col: line_col.column})
        }
        
        else if state_machine_model.top_state.evt_type_alias.is_none(){
            let line_col = trait_impl.span().start();
            Err(Error::MissingEvtTypeDef{line: line_col.line, col: line_col.column})
        }
        else{
            Ok(())        
        }
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
                    TraitImplSignatureInfo::State { state_tag, state_machine_name, impl_body } => {
                        Self::check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        Self::fill_state_model(state_tag,impl_body, &mut state_machine_model);
                    }
                    TraitImplSignatureInfo::TopState {state_machine_name, impl_body} => {
                        Self::check_state_machine_ownership(state_machine_name, &mut state_machine_model)?;
                        Self::fill_top_state_model(trait_impl, &mut state_machine_model)?;
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

