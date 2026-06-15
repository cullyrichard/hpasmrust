use std::panic;
use std::collections::HashMap;

pub fn pass_two(instructions: Vec<(String, String, String, u16)>, symbol_table: HashMap<String, u16>) -> Vec<(u16,u16)> {

let mut opcode_addr_pair: Vec<(u16, u16)> = vec![];
let mut location = 0; 

let mut mri: HashMap<&str, u16> = HashMap::new(); 
   

        mri.insert("ADA", 0o040000);
        mri.insert("ADB", 0o044000);
        mri.insert("AND", 0o010000);
        mri.insert("CPA", 0o050000);
        mri.insert("CPB", 0o054000);
        mri.insert("IOR", 0o030000);
        mri.insert("ISZ", 0o034000);
        mri.insert("JMP", 0o024000);
        mri.insert("JSB", 0o014000);
        mri.insert("LDA", 0o060000);
        mri.insert("LDB", 0o064000);
        mri.insert("STA", 0o070000);
        mri.insert("STB", 0o074000);
        mri.insert("XOR", 0o020000);

    let mut non_mri = HashMap::new();
    
        non_mri.insert( "ALF", 0o001700);
        non_mri.insert( "ALR", 0o001400);
        non_mri.insert( "ALS", 0o001000);
        non_mri.insert( "ARS", 0o001100);
        non_mri.insert( "BLF", 0o005700);
        non_mri.insert( "BLR", 0o005400);
        non_mri.insert( "BLS", 0o005020);
        non_mri.insert( "BRS", 0o005100);
        non_mri.insert( "CLE", 0o000040);
        non_mri.insert( "ELA", 0o001600);
        non_mri.insert( "ELB", 0o005600);
        non_mri.insert( "ERA", 0o001500);
        non_mri.insert( "ERB", 0o005500);
        non_mri.insert( "NOP", 0o000000);
        non_mri.insert( "RAL", 0o001200);
        non_mri.insert( "RAR", 0o001300);
        non_mri.insert( "RBL", 0o005200);
        non_mri.insert( "RBR", 0o005300);
        non_mri.insert( "SLA", 0o000010);
        non_mri.insert( "SLB", 0o004010);
        non_mri.insert( "CLA", 0o002400);
        non_mri.insert( "CLB", 0o006400);
        non_mri.insert( "CMA", 0o000200);
        non_mri.insert( "CMB", 0o007000);
        non_mri.insert( "CCA", 0o000600);
        non_mri.insert( "CCB", 0o004600);
        non_mri.insert( "CLE", 0o000040);
        non_mri.insert( "CME", 0o000040);
        non_mri.insert( "CCE", 0o000140);
        non_mri.insert( "SZA", 0o002002);
        non_mri.insert( "SZB", 0o006002);
        non_mri.insert( "RSS", 0o002001);
        non_mri.insert( "CLF", 0o103100);
        non_mri.insert( "CLO", 0o103101);
        non_mri.insert( "INA", 0o002004);
        non_mri.insert( "SEZ", 0o002040);
        non_mri.insert( "INB", 0o006004);
        non_mri.insert( "SSA", 0o002020);
        non_mri.insert( "SSB", 0o006020);
    
    let mut io = HashMap::new(); 
    
        io.insert( "HLT", 0o102000);
        io.insert( "LIA", 0o102500);
        io.insert( "LIB", 0o106500);
        io.insert( "MIA", 0o102400);
        io.insert( "MIB", 0o106400);
        io.insert( "OTA", 0o102600);
        io.insert( "OTB", 0o106600);
        io.insert( "SFC", 0o102200);
        io.insert( "SFS", 0o102300);
        io.insert( "SOC", 0o102201);
        io.insert( "SOS", 0o102301);
        io.insert( "STC", 0o102700);
        io.insert( "STF", 0o102100);
        io.insert( "STO", 0o102101);
        io.insert( "CLO", 0o103101);
        io.insert( "CLC", 0o106700);

    let mut eamr = HashMap::new(); 
    
        eamr.insert( "DIV", 0o100400);
        eamr.insert( "DLD", 0o104200);
        eamr.insert( "DST", 0o104400);
        eamr.insert( "MPY", 0o100200);

    let mut earr = HashMap::new(); 
        earr.insert( "ASR", 0o101020);
        earr.insert( "ASL", 0o100020);
        earr.insert( "LSR", 0o101040);
        earr.insert( "LSL", 0o100040);
        earr.insert( "RRR", 0o101100);
        earr.insert( "RRL", 0o100100);

    let mut itw =  HashMap::new(); 
        itw.insert( "TBS", 0o105775);
        itw.insert( "SBS", 0o105774);
        itw.insert( "CBS", 0o105776);

    //let mut itw: HashMap::new(); 
        //itw.insert()

    let mut idw = HashMap::new(); 

        idw.insert( "ADX", 0o105746);
        idw.insert( "ADY", 0o105756);
        idw.insert( "LAX", 0o101742);
        idw.insert( "LAY", 0o101752);
        idw.insert( "LBX", 0o105742);
        idw.insert( "LBY", 0o105752);
        idw.insert( "LDX", 0o105745);
        idw.insert( "LDY", 0o105755);
        idw.insert( "SAX", 0o101740);
        idw.insert( "SAY", 0o101750);
        idw.insert( "STX", 0o105743);
        idw.insert( "STY", 0o105753);
        idw.insert( "JLY", 0o105762);
        idw.insert( "JPY", 0o105772);
        idw.insert( "CBT", 0o105766);
        idw.insert( "MBT", 0o105765);
        idw.insert( "XSB", 0o105725);
        idw.insert( "XSA", 0o101725);
        idw.insert( "XMS", 0o105721);
        idw.insert( "CMW", 0o105776);
        idw.insert( "MVW", 0o105777);
        idw.insert( "FAD", 0o105000);
        idw.insert( "FDV", 0o105060);
        idw.insert( "FMP", 0o105040);
        idw.insert( "FSB", 0o105020);
        idw.insert( "XLA", 0o105722);
        idw.insert( "XLB",  0o105724);  

    let mut isw = HashMap::new(); 
        
        isw.insert( "CAX", 0o101741);
        isw.insert( "CAY", 0o101751);
        isw.insert( "CBX", 0o105741);
        isw.insert( "CBY", 0o105751);
        isw.insert( "CXA", 0o101744);
        isw.insert( "CXB", 0o105754);
        isw.insert( "CYA", 0o101754);
        isw.insert( "CYB", 0o105754);
        isw.insert( "DSX", 0o105761);
        isw.insert( "DSY", 0o105771);
        isw.insert( "ISX", 0o105760);
        isw.insert( "ISY", 0o105770);
        isw.insert( "XAX", 0o101747);
        isw.insert( "XAY", 0o101757);
        isw.insert( "XBX", 0o105747);
        isw.insert( "XBY", 0o105757);
        isw.insert( "LBT", 0o105763);
        isw.insert( "SFB", 0o105767);
        isw.insert( "SBT", 0o105764);
        isw.insert( "XMA", 0o101722);
        isw.insert( "XMM", 0o105720);
        isw.insert( "XMB", 0o105722);
        isw.insert( "FIX", 0o105120);
        isw.insert( "FLT", 0o105120);
    
for line in instructions { 
    //let label = line.0.as_str(); 
    let mnemonic = line.1.as_str(); 
    let operand = line.2.as_str(); 
    let flag = line.3; 
    
 

    if mnemonic == "ORG" || mnemonic == "org" {

    location = symbol_table.get(&mnemonic.to_string()).copied().unwrap_or(0);
    if location > 0{ 
        location = location -1; 

    }
    }
    if mnemonic == "OCT" || mnemonic == "oct" {
        
        opcode_addr_pair.push((location,u16::from_str_radix(operand, 8).unwrap_or(symbol_table.get(&operand.to_string()).copied().unwrap_or(0)))); 
    }
    if mnemonic == "DEF" || mnemonic == "def"{
     opcode_addr_pair.push((location,symbol_table.get(&operand.to_string()).copied().unwrap_or(0))); 
    }
    if mnemonic == "DEC" || mnemonic == "dec" {
        opcode_addr_pair.push((location, operand.trim().parse().unwrap()));
    }
    if mnemonic == "BSS" || mnemonic == "bss"{ 
        opcode_addr_pair.push((location, 0)); 
    }
    if mnemonic == "ABS" || mnemonic == "abs"{
        if &operand == &"var"{
        opcode_addr_pair.push((location, 0)); 
        }
        else{
         opcode_addr_pair.push((location,symbol_table.get(&operand.to_string()).copied().unwrap_or(0))); 
        }
    }
    if mnemonic == "END"{
        break ; 
    }


    if mri.contains_key(mnemonic){
        let opcode_value: u16 = mri.get(mnemonic).copied().unwrap_or(0);
        let operand_value: u16;
        let rel_operand_value: u16;
        let sub_flag:bool; 
        let temp_operand:String; 
        
        if operand.contains("*"){
            if operand.contains("-"){
                sub_flag = true; 
            }

            else{
                sub_flag = false; 
            }

            if operand.contains("+"){    
                temp_operand = operand.replace("*+", ""); 
            }

            else if operand.contains("-"){    
                temp_operand = operand.replace("*-", ""); 
            }

            else{
                temp_operand = operand.replace("*", ""); 
            }

            rel_operand_value = u16::from_str_radix(&temp_operand,8).unwrap_or(0);  
            
            if sub_flag == true {
                operand_value = location - rel_operand_value; 

            }

            else{
                operand_value = location + rel_operand_value;
            }
            
        }
        
        else{
            operand_value = u16::from_str_radix(operand, 8).unwrap_or(symbol_table.get(&operand.to_string()).copied().unwrap_or(1)-1); 
       }
        if operand_value > 0o1777{ 
            panic!("operand out of range at loaction: {:o}", location); 
        }
        opcode_addr_pair.push((location, opcode_value| operand_value | flag )); 
    }
    


    if non_mri.contains_key(mnemonic){ 
          let opcode_value: u16 = non_mri.get(mnemonic).copied().unwrap_or(0);
          let operand_value: u16 = non_mri.get(operand).copied().unwrap_or(0); 
        opcode_addr_pair.push((location, opcode_value | operand_value)); 
    }

    if io.contains_key(mnemonic){ 
        let opcode_value: u16 = io.get(mnemonic).copied().unwrap_or(0); 
        let operand_value = u16::from_str_radix(operand, 8).unwrap_or(symbol_table.get(&operand.to_string()).copied().unwrap_or(0)); 
    
        if operand_value > 0o100{
            panic!("IO Address Value Out Of Range"); 

        }

        opcode_addr_pair.push((location, opcode_value | operand_value | flag)); 
    }

    if earr.contains_key(mnemonic){ 
        let opcode_value: u16 = earr.get(mnemonic).copied().unwrap_or(0); 
        let operand_value = u16::from_str_radix(operand, 8).unwrap_or(symbol_table.get(&operand.to_string()).copied().unwrap_or(0)); 
    
        if operand_value > 0o17{
            panic!("Shift Value Out Of Range"); 

        }

        opcode_addr_pair.push((location, opcode_value | operand_value)); 
    }

    if isw.contains_key(mnemonic){
        let opcode_value = isw.get(mnemonic).copied().unwrap_or(0); 
        opcode_addr_pair.push((location,opcode_value)); 
    }

    if idw.contains_key(mnemonic) || eamr.contains_key(mnemonic){ 

        let opcode_value = idw.get(mnemonic).copied().unwrap_or(eamr.get(mnemonic).copied().unwrap_or(0));
        let operand_value =  u16::from_str_radix(operand, 8).unwrap_or(symbol_table.get(&operand.to_string()).copied().unwrap_or(0)); 
        opcode_addr_pair.push((location, opcode_value)); 
        location +=1;
        opcode_addr_pair.push((location, operand_value|flag)); 
    }
    
    location = location + 1;
}


    return opcode_addr_pair; 
    


}

