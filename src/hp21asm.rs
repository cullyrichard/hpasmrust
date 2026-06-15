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
    let mut list_path:String; 
    let args: Vec<String> = env::args().collect(); 

    let mut list_flag:bool = false; 

    for element in &args{
        if element.contains("-l"){
            list_flag = true; 
        }
        if element.contains("-h"){
         println!("\nUSAGE: hp21asm path_to_source path_to_output\nIf path_to_output is left blank, the assembler will create a output.abs file in the same directory as the input file"); 
         return; 
        }

    }

    if args.len() == 3 || (list_flag == true && args.len() == 4) {
        source_path = (&args[1]).to_string(); 
        output_path = (&args[2]).to_string(); 
            if list_flag == true {
                list_path = output_path.replace("asm","lst"); 
            }
    }
    else if args.len() == 2 || (list_flag == true && args.len() == 3){ 
        source_path = (&args[1]).to_string();  
    }


    else{
        println!("\nUSAGE: hp21asm path_to_source path_to_output \nuse -h for help ");
        return; 
    }
    let source: Vec<(String, String, String, u16)> = parse::parse_line(file_string::file_string(&source_path));
    let pass_two_out = pass_two::pass_two(source.clone(), pass_one(source.clone())); 
    if list_flag == true{
        let mut source_line = 0; 
        for line in &pass_two_out{
            let (string1,string2,string3,line4) = &source[source_line]; 
            let source_line_combined = string1.to_owned()+" " + string2 + " " +string3;
            let &(location, instruction) = line; 
            println!("{:06o} | {:06o} | {:?}", location,instruction,source_line_combined); 
            source_line += 1; 
        }
    }

    abs(pass_two_out, output_path);
} 