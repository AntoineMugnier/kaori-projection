use std::ops::Deref;
use crate::errors::Error;
use syn::{ImplItemFn, spanned::Spanned};
use crate::model;

pub fn parse_match_arm_body(body: &syn::Expr) -> Result<(Vec<model::Next>, Option<String>), Error>{
        let tree = Vec::<model::Next>::new();
        let action = None;
        if let syn::Expr::Block(body) = body{
            let stmts = &body.block.stmts;
            let mut action = None;
            for stmt in stmts{
                //println!("{:#?}", stmt);
                if let syn::Stmt::Expr(expr,_ ) = stmt {
                   
                }
                action = stmt.span().source_text(); // Get the whole statement as a string 
            }
        }
        Ok((tree, action))    
    }

    pub fn parse_handle(fn_: &ImplItemFn)-> Result<model::EvtHandler, Error> {
        let stmts = &fn_.block.stmts;
        let mut evt_handler = model::EvtHandler::new();

        for stmt in stmts{
            if let syn::Stmt::Expr(expr,_ ) = stmt {
                match expr{
                    syn::Expr::Match(match_) =>{
                        
                        let mut correct_matched_arm = false;
                        // Check that the match is being done on the proper variable
                        if let syn::Expr::Path(evt_name) = match_.expr.deref(){
                           if let Some(evt_name) = evt_name.path.get_ident(){
                                if evt_name.to_string() == "evt"{
                                    correct_matched_arm = true;
                                }
                            }
                        }

                        if !correct_matched_arm { 
                            continue;
                        }

                        for arm in match_.arms.iter(){
                            let mut evt_catcher = model::EvtCatcher::new();
                            let mut correct_variant = false;
                           let pat = &arm.pat;
                            
                            match pat{
                                syn::Pat::Wild(_) => {break;}
                                syn::Pat::Path(path)=>{
                                    let segments = &path.path.segments;
                                    let segments_vec_len = segments.len();
                                    if segments_vec_len>=2{
                                        let evt_type_name = &segments[segments_vec_len - 2];
                                        let evt_variant_name = &segments[segments_vec_len - 1];
                                        evt_catcher.evt_type_name = evt_type_name.ident.to_string();
                                        evt_catcher.evt_variant_name = evt_variant_name.ident.to_string();
                                        correct_variant = true;
                                    }
                                }
                                _ => {continue;}
                            }
                       //println!("{:#?}",arm);
                            if correct_variant == false{
                                let line_col = pat.span().start();
                                return Err(Error::InvalidEvtMatch{ line:line_col.line , col: line_col.column});
                            }
                            let body = arm.body.deref(); 
                            (evt_catcher.next, evt_catcher.action) = parse_match_arm_body(&body)?;
                            evt_handler.evt_catchers.push(evt_catcher);
                        } 
                    }
                    _ => (),
                }
            }
        }

        Ok(evt_handler)
    }
