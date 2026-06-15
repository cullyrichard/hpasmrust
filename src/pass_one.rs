use std::collections::HashMap;

pub fn pass_one(parsed_input:Vec<(String,String,String, u16)>) -> HashMap<String, u16> { // build the symbol table 
    
    let mut sym_tab: HashMap<String, u16> = HashMap::new();
    let mut location_counter = 0; 

    for line in parsed_input{ 
        let label: String = line.0; 
        let mnemonic: String = line.1; 
        let mut operand: String = line.2; 
        
        if mnemonic == "ORG" || mnemonic == "org"{ 
            if operand.contains("b"){
                operand.pop(); 
                location_counter = u16::from_str_radix(operand.trim(), 2).unwrap();
                sym_tab.insert(mnemonic.to_string() , location_counter); 
            }
            else{

            
            location_counter = u16::from_str_radix(operand.trim(), 8).unwrap();
            sym_tab.insert(mnemonic.to_string() , location_counter); 
            //println!("{:o}", location_counter)
            }
        } 
        if !label.is_empty() { 
            sym_tab.insert(label,location_counter); 

        }
        location_counter += 1; 

    }

    return sym_tab;
}
