use std::fs; 
pub fn abs(inst_addr_vec:Vec<(u16,u16)>, output_path:String){
    let mut abs_vector:Vec<u8> = vec![]; 

    let mut addr:u16 = inst_addr_vec[0].0; 
    let chunk_size = 255; 
     


 
    for instruction_chunk in inst_addr_vec.chunks(chunk_size){
        //println!("{:?}", addr); 
        //println!("{:?}", instruction_chunk.len()); 
        let inst_block_len = instruction_chunk.len() as u8;
        let [addr_high_byte, addr_low_byte] = addr.to_be_bytes(); 
        let mut checkword:u16 = addr; 

            abs_vector.push(inst_block_len); 
            abs_vector.push(0); 
            abs_vector.push(addr_high_byte); 
            abs_vector.push(addr_low_byte); 

            for element in instruction_chunk{ 
                let [high_inst_byte, low_inst_byte] = element.1.to_be_bytes(); 
                abs_vector.push(high_inst_byte); 
                abs_vector.push(low_inst_byte); 
                checkword = checkword.wrapping_add(element.1); 
            }
            let [checkword_high_byte, checkword_low_byte] = checkword.to_be_bytes(); 
            abs_vector.push(checkword_high_byte);
            abs_vector.push(checkword_low_byte);  

            addr = addr + (inst_block_len as u16); 
            
            for _i in 0..20{
                abs_vector.push(0); 
            }
            
    }



 //   for instruction in inst_addr_vec{ 

 //       let [inst_high_byte, inst_low_byte] = instruction.1.to_be_bytes(); 



   // }

fs::write(output_path, abs_vector); 
//return abs_vector; 

}

