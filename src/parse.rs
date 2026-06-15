pub fn parse_line(mut line_vector: Vec<String>) -> Vec<(String, String, String, u16)>{
    let mut label: &str; 
    let mut mnemonic: &str;
    let mut operand: &str; 
    let mut output_vector: Vec<(String, String, String,u16)> = vec![]; 
    let mut flag: u16 = 0;  

    line_vector.retain(|s| !s.starts_with("*")); 
    line_vector.retain(|s| !s.trim().is_empty()); // remove blank lines before we start parsing.  

    

    for mut line_selected in line_vector{

            line_selected = line_selected.replace("\r", " ").replace("\t"," ");  // strip out \r and \t
            //println!("{:?}", line_selected); 
            let line_split:Vec<&str> = line_selected.as_str().split(" ").collect();  
            //println!("{:?}",line_split); 
            let label_tag = line_split[0].contains("_"); 
            
            //println!("{:?}",line_split.len());
            
            if line_split.len() < 4 && !(label_tag){        
               label = ""; 
                mnemonic = line_split[0]; 
                operand = line_split[1]; 
            }   
            
            else {
                label = line_split[0]; 
                mnemonic = line_split[1]; 
                operand = line_split[2];
            }            
         
            if mnemonic == ""{
                mnemonic = line_split[2]; 
                operand = line_split[3]; 
            }
            
            if operand.contains(","){ 

                if operand.contains(",I"){
                    flag = flag | 1<<15; 
                }
                if operand.contains(",D"){
                    flag = flag | 1<<10;  
                }
                if operand.contains(",C"){
                    flag = flag | 1<<9; 
                }
                let split_operand: Vec<&str> = operand.split(",").collect(); 
                operand = split_operand[0]; 
            }


            if mnemonic.contains(","){
                let non_mri_split:Vec<&str> = mnemonic.split(",").collect();
                mnemonic = non_mri_split[0]; 
                operand = non_mri_split[1];  
            }

          


            output_vector.push((label.to_string(), mnemonic.replace("\r", "").replace("\t","").to_string(), operand.replace("\r", "").replace("\t","").to_string(), flag));
            flag = 0; //reset flag for next line 
        }
    
//println!("{:?}", output_vector);
    return output_vector;
}
