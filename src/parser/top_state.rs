use crate::{errors::Error, model::StateMachine, model, parser, parser::{TraitImplSignatureInfo, init}};
use syn::{Type, ImplItem, ImplItemType, ItemImpl, spanned::Spanned};

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
                        state_tag: parser::get_state_tag(&segment.arguments)?,
                        state_machine_name : get_state_type(&trait_impl.self_ty)?,
                };
                    
            }
            else if trait_impl_ident.to_string() == "TopState"{
                trait_impl_signature_info = TraitImplSignatureInfo::TopState {
                state_machine_name: get_state_type(&trait_impl.self_ty)?,
                }
            }
            else{
                trait_impl_signature_info = TraitImplSignatureInfo::Other;
            }
            return Ok(trait_impl_signature_info);
        }
        panic!("Should not happen")
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

    pub fn fill_top_state_model(trait_impl : &ItemImpl, state_machine_model: &mut model::StateMachine) -> Result<(), Error>{
        let state_trait_impl_body =  &trait_impl.items;
        for item in state_trait_impl_body.iter(){
            match item{
                ImplItem::Type(type_) =>{
                     parse_top_state_associated_type(&type_, state_machine_model)?;
                },
                ImplItem::Fn(fn_) =>{
                    if fn_.sig.ident.to_string() == "init"{
                        let (init_target, action) = init::parse_init_fn(fn_)?;
                        state_machine_model.top_state.init.target = init_target;
                        state_machine_model.top_state.init.action = action;
                    }
                },
                _ =>{}
            }
        }
        
        // Trigger error if all searched items are not found
        if state_machine_model.top_state.init.target.is_empty(){
            let line_col = trait_impl.span().start();
            Err(Error::MissingInitDef{line: line_col.line, col: line_col.column})
        }
        
        else if state_machine_model.top_state.evt_type_alias.is_none(){
            let line_col = trait_impl.span().start();
            Err(Error::MissingEvtTypeDef{line: line_col.line, col: line_col.column})
        }
        else{
            Ok(())        
        }
    }

