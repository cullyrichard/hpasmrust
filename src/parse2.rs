pub fn parse_line2(line_vector: Vec<String>) -> Vec<(String,String,String,u16)>{
let mut label: &str; 
let mut mnemonic: &str; 
let mut operand: &str; 
let mut output_vector: Vec<(String,String,String,u16)> = vec![]; 
let mut flag: u16; 
let mut line: &str; 






return output_vector; 

}
fn strip_lines(lines: Vec<String>) -> Vec<String> {
let mut output_line: Vec<String> = vec![]; 

for line_iter in lines{ 

output_line.push(line_iter.replace(&['\t', '*', '\n'], " "));


}


return output_line; 


}