use crate::errors::Error;
use syn::{ImplItemFn, spanned::Spanned};
use std::ops::Deref;

   fn try_parse_init_exprmacro(exprmacro: &syn::ExprMacro) -> Option<String>{
         if let Some(last_path_segment) = exprmacro.mac.path.segments.last(){
            if last_path_segment.ident.to_string() == "init_transition"{
                let target_state_tag = &exprmacro.mac.tokens.to_string();
                return Some(target_state_tag.clone());
            }
        }
        return None;
    }

    pub fn parse_init_fn(fn_: &ImplItemFn) -> Result<(String, Option<String>), Error>{
        let stmts = &fn_.block.stmts;
        let mut action = None;
        for stmt in stmts{
            if let syn::Stmt::Expr(expr,_ ) = stmt {
                match expr{
                   syn::Expr::Macro(exprmacro) => {
                        if let Some(target_state_tag) = try_parse_init_exprmacro(exprmacro){
                            return Ok((target_state_tag.clone(), action));
                        }
                    }
                    syn::Expr::Return(expreturn) =>{
                        let expr = &expreturn.expr;
                        if let Some(expr) = expr{
                            if let syn::Expr::Macro(exprmacro) = expr.deref() {
                                if let Some(target_state_tag) = try_parse_init_exprmacro(exprmacro){
                                return Ok((target_state_tag.clone(), action));

                                } 
                            }
                        }
                    }
                    _ => (),
                }
            }
            action = stmt.span().source_text(); // Get the whole statement as a string 
        }
        
       let line_col = fn_.span().start();
       return Err(Error::MissingInitTranCall{line: line_col.line, col: line_col.column})
    }
