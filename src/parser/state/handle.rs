use std::ops::Deref;
use crate::{errors::Error};
use syn::{ImplItemFn, spanned::Spanned};
use crate::model;
enum MacroType{
    InitTransition{target: String},
    Transition{target: String},
    Handled,
    Ignored,
    Other
}   
    fn try_parse_exprmacro(exprmacro: &syn::ExprMacro) -> MacroType{
         let last_path_segment = exprmacro.mac.path.segments.last().unwrap();
            let ident = last_path_segment.ident.to_string();
            match ident.as_str(){
                "init_transition" =>{
                    let target_state_tag = exprmacro.mac.tokens.to_string();
                    return MacroType::InitTransition { target: target_state_tag};
                }
                "transition" =>{
                    let target_state_tag = exprmacro.mac.tokens.to_string();
                    return MacroType::Transition { target: target_state_tag};
                }
                "handled" => return MacroType::Handled,
                "ignored" => return MacroType::Ignored,
                _ => return MacroType::Other
            }
    }

   enum StatementType {
    Action(String),
    Next(model::Next),
    Block(model::Next, Option<String>)
   }
    
    fn match_exprmacro(exprmacro: &syn::ExprMacro) -> Result<model::Next, Error>{
            match try_parse_exprmacro(&exprmacro){
               MacroType::Transition { target } =>{
                    let transition_target = model::TransitionTarget{state_name: target};
                    return Ok(model::Next::Target(transition_target));
               }
                MacroType::Handled =>{
                    return Ok(model::Next::Handled());
               }
                _ =>{ unimplemented!()}
            }
    }

    fn match_handle_expr(expr: &syn::Expr) -> Result<StatementType, Error>{
       match expr{
        syn::Expr::If(ifexpr) =>{
             let condition = parse_condition(ifexpr)?;
             return Ok(StatementType::Next(model::Next::Condition(condition)));
        }
        syn::Expr::Macro(exprmacro) => {
            let next = match_exprmacro(exprmacro)?;
            return Ok(StatementType::Next(next));
        }
        syn::Expr::Return(expreturn) =>{
            let expr = &expreturn.expr;
            if let Some(expr) = expr{
                if let syn::Expr::Macro(exprmacro) = expr.deref() {
                    let next = match_exprmacro(exprmacro)?;
                    return Ok(StatementType::Next(next));
                } 
            }
        }
        syn::Expr::Block(exprblock) =>{
            let block = &exprblock.block;
            let (branch, action) = parse_block(block)?;
            return Ok(StatementType::Block(branch, action));

        }
        _ => {
            let action = expr.span().source_text(); // Get the whole statement as a string 
            if let Some(action) = action{
                return Ok(StatementType::Action(action))
            }
        }
        }
       
        unimplemented!() //Error
    }

    pub fn parse_block(body: &syn::Block) -> Result<(model::Next, Option<String>), Error>{
    let stmts = &body.stmts;
        let mut block_action = None;
        for stmt in stmts{
            match stmt{
                syn::Stmt::Expr(expr,_ ) =>{
                    let statement_type = match_handle_expr(expr)?;
                    match statement_type{
                      StatementType::Action(action) => {
                        block_action = Some(action);
                      }
                      StatementType::Next(next) =>{
                        return Ok((next, block_action));
                      }
                      StatementType::Block(branch, action) =>{
                            if action.is_some(){
                                block_action = action.clone();
                            }
                            if branch != model::Next::Unterminated(){
                                return Ok((branch, action));
                            }
                        }
                    }
                },
               // Any statement not recognized is parsed as an action 
                _=> block_action = stmt.span().source_text()
            }
        }
        return Ok((model::Next::Unterminated(), block_action));
    }

    fn parse_condition(if_expr: &syn::ExprIf) -> Result<model::Condition, Error> {
       let mut conditional_branches = Vec::new();

       if let Some(guard) = if_expr.cond.span().source_text(){
        let (branch, action) = parse_block(&if_expr.then_branch)?;
        let conditional_branch = model::ConditionalBranch{guard, action, next : branch};
        conditional_branches.push(conditional_branch);
       }
       if let Some(else_branch) = &if_expr.else_branch{
        let expr = else_branch.1.deref();
        match expr{
            syn::Expr::Block(block) =>{
                let (branch, action) = parse_block(&block.block)?;
                let conditional_branch = model::ConditionalBranch{guard: String::from("else"), action, next : branch};
                conditional_branches.push(conditional_branch);
            }
            syn::Expr::If(ifexpr) =>{
                let mut condition = parse_condition(ifexpr)?;
                conditional_branches.append(&mut condition.branches);
            }
            _ => unimplemented!()
        } 
       }
       
       let condition = model::Condition{branches: conditional_branches};
       return Ok(condition);
    }

    pub fn parse_match_arm_body(body: &syn::Expr) -> Result<(model::Next, Option<String>), Error>{
        if let syn::Expr::Block(body) = body{
            let (next, action) = parse_block(&body.block)?;
            return Ok((next, action));    
        }
        unimplemented!()

    }


#[cfg(test)]
mod tests {
use crate::string;

    use super::*;
    use model::*;
    fn test_parse_match_arm_body_cmp(input: &str, expected: (model::Next, Option<String>)){
        let stream : proc_macro2::TokenStream = input.parse().unwrap();
        let ast: syn::Arm = syn::parse2(stream).unwrap();
        let obtained_res = parse_match_arm_body(ast.body.deref()).unwrap();
        if obtained_res != expected{
            eprintln!(" EXPECTED \n{:#?}", expected);
            eprintln!("---------------------------------");
            eprintln!(" GOT \n{:#?}", obtained_res);
            panic!("Parsing results do not match")
        }
    }

    #[test]
    fn test_parse_match_arm_body(){
        {
            let code_to_be_parsed = r#"
            BasicEvt::A =>{
                let a: u8 = 0;
                if a==3 && call_x() {
                    call_fn();
                    transition!(S2)
                }
                else if a == 2{
                    return handled!()
                }
                else{
                    println!("S0-HANDLES-A");
                    return transition!(S1)
                }
            } 
            "#;

            let expected_parsing_result = (
                Next::Condition(
                    Condition {
                        branches: vec![
                            ConditionalBranch{
                                guard: string!("a==3 && call_x()"),
                                action: Some(string!("call_fn()")),
                                next: Next::Target(
                                    TransitionTarget{state_name: string!("S2")}
                                )
                            },
                            ConditionalBranch{
                                guard: string!("a == 2"),
                                action: None,
                                next: Next::Handled()
                            },
                            ConditionalBranch{
                                guard: string!("else"),
                                action: Some(string!("println!(\"S0-HANDLES-A\");")),
                                next: Next::Target(
                                    TransitionTarget { state_name: string!("S1")}
                                )
                            }
                        ]
                    }
                ),
                Some(string!("let a: u8 = 0;"))
            );
            
            test_parse_match_arm_body_cmp(code_to_be_parsed, expected_parsing_result);
        }
        {
            let code_to_be_parsed = r#"
            BasicEvt::A =>{
                if a{
                    transition!(S2)
                }
                else if !a{
                   {
                        call_fn();
                    }
                    return handled!()
                }
            } 
            "#;

            let expected_parsing_result = (
                Next::Condition(
                    Condition {
                        branches: vec![
                            ConditionalBranch{
                                guard: string!("a"),
                                action: None,
                                next: Next::Target(
                                    TransitionTarget{state_name: string!("S2")}
                                )
                            },
                            ConditionalBranch{
                                guard: string!("!a"),
                                action: Some(string!("call_fn()")),
                                next: Next::Handled()
                            }
                        ]
                    }
                ),
                None
            );

            test_parse_match_arm_body_cmp(code_to_be_parsed, expected_parsing_result);
        }
    }
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
                            let pat = &arm.pat;
                            let evt_variant_name;
                            let evt_type_name;
                            match pat{
                                syn::Pat::Wild(_) => {break;}
                                syn::Pat::Path(path)=>{
                                    let segments = &path.path.segments;
                                    let segments_vec_len = segments.len();
                                    if segments_vec_len>=2{
                                        let evt_type_name_ps = &segments[segments_vec_len - 2];
                                        evt_type_name = evt_type_name_ps.ident.to_string();
                                        let evt_variant_name_ps = &segments[segments_vec_len - 1];
                                        evt_variant_name = evt_variant_name_ps.ident.to_string();
                                    }
                                    else{
                                        let line_col = pat.span().start();
                                        return Err(Error::InvalidEvtMatch{ line:line_col.line , col: line_col.column});
                                    }
                                }
                                _ => {continue;}
                            }
                            let body = arm.body.deref(); 
                            let (next, action) = parse_match_arm_body(&body)?;
                            let evt_catcher = model::EvtCatcher { evt_type_name, evt_variant_name, action, next};
                            evt_handler.evt_catchers.push(evt_catcher);
                        } 
                    }
                    _ => (),
                }
            }
        }

        Ok(evt_handler)
    }
