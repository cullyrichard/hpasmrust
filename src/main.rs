use crate::{abs::abs, pass_one::pass_one};
use std::env;
mod file_string; 
mod parse; 
mod pass_one;
mod pass_two; 
mod abs; 


 fn main(){ 
    let source_path:String; 
    let mut output_path:String = "ouput.abs".to_string(); 

    let args: Vec<String> = env::args().collect(); 

    if args.len() > 1 && args[1] == "-h" {
         println!("\nUSAGE: hp21asm path_to_source path_to_output\nIf path_to_output is left blank, the assembler will create a output.abs file in the same directory as the input file"); 
         return; 
    }
    if args.len() > 3 {
        source_path = (&args[1]).to_string(); 
        output_path = (&args[2]).to_string(); 
    }
    else if args.len() < 3 && args.len() > 1 { 
        source_path = (&args[1]).to_string();  
    }


    else{
        println!("\nUSAGE: hp21asm path_to_source path_to_output \nuse -h for help ");
        return; 
    }

    let source: Vec<(String, String, String, u16)> = parse::parse_line(file_string::file_string(&source_path));
    abs(pass_two::pass_two(source.clone(), pass_one(source.clone())), output_path);
} 