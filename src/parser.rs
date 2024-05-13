use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Parser{
    file_path : String    
}

impl Parser{

    pub fn new(file_path: &String) -> Parser{
        Parser{file_path: file_path.clone()}
    }

    pub fn parse(& self ) -> Result<(), Box<dyn Error>>{
        // Extract file content to string
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new(); 
        file.read_to_string(&mut content)?;

        // Convert string to AST
        let ast = syn::parse_file(&content)?;
        println!("{:?}", ast);
        Ok(())
    }

}
