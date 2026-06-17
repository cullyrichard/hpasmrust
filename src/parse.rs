pub fn parse_line(mut line_vector: Vec<String>) -> Vec<(String, String, String, u16)>{
    let mut label:String;
    let mut mnemonic:String;
    let mut operand:String;  
    let mut output_vector:Vec<(String, String, String,u16)> = vec![]; 
    let mut line_split:Vec<&str>; 
    let mut flag:u16 = 0;  

    
    let instructions = ["ADA", "ADB", "AND", "CPA", "CPB", "IOR", "ISZ", "JMP", "JSB", "LDA", "LDB", "STA", "STB", "XOR", "ALF", 
     "ALR", "ALS", "ARS", "BLF", "BLR", "BLS", "BRS", "CLE", "ELA", "ELB", "ERA", "ERB", "NOP", "RAL", "RAR", 
     "RBL", "RBR", "SLA", "SLB", "CLA", "CLB", "CMA", "CMB", "CCA", "CCB", "CLE", "CME", "CCE", "SZA", "SZB", 
     "RSS", "CLF", "CLO", "INA", "SEZ", "INB", "SSA", "SSB", "HLT", "LIA", "LIB", "MIA", "MIB", "OTA", "OTB", 
     "SFC", "SFS", "SOC", "SOS", "STC", "STF", "STO", "CLO", "CLC", "DIV", "DLD", "DST", "MPY",  "ASR", "ASL", 
     "LSR", "LSL", "RRR", "RRL", "TBS", "SBS", "CBS", "ADX", "ADY", "LAX", "LAY", "LBX", "LBY", "LDX", "LDY",  
     "SAX", "SAY", "STX", "STY", "JLY", "JPY", "CBT", "MBT", "XSB", "XSA", "XMS", "CMW", "MVW", "FAD", "FDV", 
     "FMP", "FSB", "XLA", "XLB",  "CAX", "CAY", "CBX", "CBY", "CXA", "CXB", "CYA", "CYB", "DSX", "DSY", "abs", "ABS", 
     "ISX", "ISY", "XAX", "XAY", "XBX", "XBY", "LBT", "SFB", "SBT", "XMA", "XMM", "XMB", "FIX", "FLT", "ORG", "org", 
     "oct", "OCT", "dec", "DEC","equ","EQU"]; 
    
    
    
    line_vector.retain(|s| !s.starts_with("*")); 
    line_vector.retain(|s| !s.trim().is_empty()); // remove blank lines before we start parsing.  

    





    for line_selected in line_vector{

            let line_intermediate = line_selected.replace("\r", " ").replace("\t"," ").to_string();  // strip out \r and \t
            line_split =  line_intermediate.as_str().split_whitespace().collect(); 
            //let label_tag = line_split[0].contains("_"); 
            
            let mut str_index_iter:u32 = 0; 
            let mut str_index:u32 = 0; 

            for line_element in &line_split{ 
                str_index_iter +=1; 
                if instructions.iter().any(|&instruction| line_element.contains(instruction)){
                    str_index = str_index_iter; 
                    break; 
                }
            }
            //println!("{:?}", &str_index); 
            
            if str_index == 1 && line_split.len() != 1{ 
                label = "".to_string(); 
                mnemonic = line_split[(str_index -1) as usize].to_string(); 
                operand =  line_split[(str_index )as usize].to_string(); 
            }
            else if str_index == 1&& line_split.len() == 1{ 
                label = "".to_string(); 
                mnemonic = line_split[(str_index -1) as usize].to_string(); 
                operand = "".to_string(); 
            }
            else if str_index  == 2 && line_split.len() > 2 {
                label = line_split[(str_index -2) as usize].to_string(); 
                mnemonic = line_split[(str_index -1) as usize].to_string(); 
                operand =  line_split[(str_index )as usize].to_string();
            }
            else if str_index == 2 && line_split.len() == 2{
                label = line_split[(str_index -2) as usize].to_string(); 
                mnemonic = line_split[(str_index -1) as usize].to_string(); 
                operand =  "".to_string();
            }
                else if str_index == 2 && line_split.len() == 1{
                label = line_split[(str_index -2) as usize].to_string(); 
                mnemonic = line_split[(str_index -1) as usize].to_string(); 
                operand =  "".to_string();
            }
            else {
                label = "".to_string();
                mnemonic = "".to_string();
                operand = "".to_string();
            }


            if mnemonic.contains(","){
                let local_mnemonic = mnemonic.clone(); 
                let non_mri_split:Vec<&str> = local_mnemonic.split(",").collect();
                mnemonic = non_mri_split[0].to_string();
                operand = non_mri_split[1].to_string();  
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
                operand = split_operand[0].to_string(); 
            }



          


            output_vector.push((label.clone(), mnemonic.clone(), operand.clone(), flag));
            flag = 0; //reset flag for next line 

        }


println!("{:?}", output_vector);
            return output_vector;
 
        }    
    //println!("{:?}", output_vector);



