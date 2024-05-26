use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{parse::Parse, PathArguments, GenericArgument, Type};

use crate::share::{State, self};

pub struct Parser{
    file_path : String    
}
#[derive(Debug)]
pub enum TraitImplHeaderInfo{
    State{state_tag : String, state_machine_name: String},
    TopState{state_machine_name: String},
    Other
}

impl Parser{

    pub fn new(file_path: &String) -> Parser{
        Parser{file_path: file_path.clone()}
    }
    
    fn get_state_tag(path_argument: &PathArguments) -> String{
        if let PathArguments::AngleBracketed(ab) = path_argument{
            if let GenericArgument::Type(generic_arg_0) = &ab.args[0]{
                if let Type::Path(path) = generic_arg_0{
                    let ident = &path.path.segments[0].ident;
                    return ident.to_string()
                } 
             }
        }
        panic!()
    }

    fn get_state_type(self_ty: &Type) -> String{
            if let Type::Path(path) = self_ty{
                    let ident = &path.path.segments[0].ident;
                    return ident.to_string()
                }
            panic!();
    }

    pub fn get_trait_impl_type(trait_impl :  &syn::ItemImpl) -> TraitImplHeaderInfo{
        let trait_impl_header_info : TraitImplHeaderInfo;
        if let Some(trait_) = &trait_impl.trait_{
            let path = &trait_.1;
            let segment = &path.segments[0];
            let trait_impl_ident = &segment.ident;
            if trait_impl_ident.to_string() == "State"{
                trait_impl_header_info = TraitImplHeaderInfo::State {
                    state_tag: Self::get_state_tag(&segment.arguments),
                    state_machine_name : Self::get_state_type(&trait_impl.self_ty)};
            }
            else if trait_impl_ident.to_string() == "TopState"{
                trait_impl_header_info = TraitImplHeaderInfo::TopState {
                    state_machine_name: Self::get_state_type(&trait_impl.self_ty) }
            }
            else{
                trait_impl_header_info = TraitImplHeaderInfo::Other;
            }
            return trait_impl_header_info;
        }
        panic!()
    }

    pub fn parse(& self ) -> Result<(Box<State>), Box<dyn Error>>{
        // Extract file content to string
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new(); 
        file.read_to_string(&mut content)?;
        
        // Convert string to AST
        let ast = syn::parse_file(&content)?;

        let states_vec = Vec::<State>::new();
        for item in ast.items.iter(){
            if let syn::Item::Impl(trait_impl) = item {
                let trait_impl_info = Self::get_trait_impl_type(&trait_impl);
                println!("{:?}", trait_impl_info);
            }
        }
        unimplemented!();

    }

}

