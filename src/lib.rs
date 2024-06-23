//0.1.8
//बिजी७७<bandesh@gmail.com>

use std::collections::HashMap;

pub struct NepaliTransliterator {
    mappings: HashMap<String, (String, VowelType)>,
    reverse_mappings :HashMap<(String, VowelType), Vec<String>>,
   
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum VowelType {
    Dependent,
    Independent,
    None,
}



impl NepaliTransliterator {
    pub fn new() -> Self {
     
        let mut mappings: HashMap<String, (String, VowelType)> = HashMap::new();
        let map_data = vec![
            ('अ', ("a", VowelType::Independent)),
            ('आ', ("ā", VowelType::Independent)),
            ('इ', ("i", VowelType::Independent)),
            ('ई', ("ī", VowelType::Independent)),
            ('उ', ("u", VowelType::Independent)),
            ('ऊ', ("ū", VowelType::Independent)),
            ('ए', ("e", VowelType::Independent)),
            ('ऐ', ("ai", VowelType::Independent)),
            ('ओ', ("o", VowelType::Independent)),
            ('औ', ("au", VowelType::Independent)),
            ('क', ("k", VowelType::None)),
            ('ख', ("kh", VowelType::None)),
            ('ग', ("g", VowelType::None)),
            ('घ', ("gh", VowelType::None)),
            ('च', ("c", VowelType::None)),
            ('छ', ("ch", VowelType::None)),
            ('ज', ("j", VowelType::None)),
            ('झ', ("jh", VowelType::None)),
            ('ञ', ("ñ", VowelType::None)),
            ('ट', ("ṭ", VowelType::None)),
            ('ठ', ("ṭh", VowelType::None)),
            ('ड', ("ḍ", VowelType::None)),
            ('ढ', ("ḍh", VowelType::None)),
            ('ण', ("ṇ", VowelType::None)),
            ('त', ("t", VowelType::None)),
            ('थ', ("th", VowelType::None)),
            ('द', ("d", VowelType::None)),
            ('ध', ("dh", VowelType::None)),
            ('न', ("n", VowelType::None)),
            ('प', ("p", VowelType::None)),
            ('फ', ("ph", VowelType::None)),
            ('ब', ("b", VowelType::None)),
            ('भ', ("bh", VowelType::None)),
            ('म', ("m", VowelType::None)),
            ('य', ("y", VowelType::None)),
            ('र', ("r", VowelType::None)),
            ('ल', ("l", VowelType::None)),
            ('व', ("v", VowelType::None)),
            ('श', ("ś", VowelType::None)),
            ('ष', ("ṣ", VowelType::None)),
            ('स', ("s", VowelType::None)),
            ('ह', ("h", VowelType::None)),
            ('ं', ("ṃ", VowelType::Dependent)),
            ('ँ', ("ṅ", VowelType::Dependent)),
            ('्', ("", VowelType::Dependent)), 
            ('ा', ("ā", VowelType::Dependent)),
            ('े', ("e", VowelType::Dependent)),
            ('ि', ("i", VowelType::Dependent)),
            ('ी', ("ī", VowelType::Dependent)),
            ('ो', ("o", VowelType::Dependent)),
            ('ु', ("u", VowelType::Dependent)),
            ('ू', ("ū", VowelType::Dependent)),
            ('ृ', ("ṛ", VowelType::Dependent)),
            ('ः', ("ḥ", VowelType::Dependent)),
        ];

        for (nepali_char, (roman_char, vowel_type)) in map_data {
            mappings.insert(
                nepali_char.to_string(),
                (roman_char.to_string(), vowel_type),
            );
        }
   
     let mut reverse_mappings: HashMap<(String, VowelType), Vec<String>> = HashMap::new();
    for (nepali, (roman, vowel_type)) in &mappings {
        reverse_mappings
            .entry((roman.clone(), *vowel_type))
            .or_insert(Vec::new())
            .push(nepali.clone());
    }                 
  /*      
  let mut reverse_mappings : HashMap<String, (String, VowelType)> = HashMap::new();
       for (nepali, (roman, vowel_type)) in &mappings {
            reverse_mappings.insert(roman.clone(), (nepali.clone(), *vowel_type));
        }
  */    
        NepaliTransliterator {
            mappings,
            reverse_mappings,
        }
        
      
    }
 
   fn is_vowel(&self, ch: &str) -> bool {
        for ((roman, vowel_type), _) in &self.reverse_mappings {
            if roman == ch && matches!(vowel_type, VowelType::Independent | VowelType::Dependent) {
               // print!("{}  is vowel ", ch);
                return true;
            }
        }
        false
    }

    fn is_consonant(&self, ch: &str) -> bool {
        for ((roman, vowel_type), _) in &self.reverse_mappings {
            if roman == ch && matches!(vowel_type, VowelType::None) {
                return true;
            }
        }
    false // Return false if character not found in mappings
    }




    pub fn to_roman(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                result.push(c); // Add whitespace character directly to result
            } else {
                let mut candidate = c.to_string();
                while let Some(&next_char) = chars.peek() {
                    let next_candidate = candidate.clone() + &next_char.to_string();
                    if self.mappings.contains_key(&next_candidate) {
                        candidate = next_candidate;
                        chars.next(); // Consume the peeked character
                    } else {
                        break;
                    }
                }
                if let Some(mapping) = self.mappings.get(&candidate) {
                    result.push_str(&mapping.0);
                } else {
                    result.push('?');
                }
            }
        }
        result
    }

   pub fn to_nepali(&self, input: &str) -> String {
        let mut result = String::new();
        let mut buffer = String::new();
        let mut chars = input.chars().peekable();
        
       // println!("Number of mappings: {} \n Mappings: {:?}", self.mappings.len(), self.mappings); 
   // println!("Number of mappings: {} \n Reverse Mappings: {:?}", self.reverse_mappings.len(), self.reverse_mappings);
        
        
        while let Some(c) = chars.next() {
         
                
              
               buffer.push(c);
               
                   if c.is_whitespace()  {  // ||  self.is_vowel(&c.to_string()) 
                    
                    result.push_str(&self.process_buffer(&buffer));
                    buffer.clear();
                }
               
               
            
            
        }//while

       

        result
    }

       fn process_buffer(&self, sub_buffer: &str) -> String {
        println!("Sub buffer: '{}'", sub_buffer);
        //println!("Reverse mappings: {:?}", self.reverse_mappings);

        if sub_buffer.is_empty() {
            return String::new(); // Return an empty string if the sub_buffer is empty
        }

        let mut result = String::new();
        let mut chars = sub_buffer.chars().peekable();
        let mut prev_was_consonant = false;
        let mut isbeginning = true;
        
        


         while chars.peek().is_some() {
             
            let mut found_mapping = false; // Reset found_mapping at the start of each iteration
            let mut chunk = String::new();
           
           
    // Try to get the first 3 characters
            chunk = chars.clone().take(3).collect();
            if let Some(mapped_value) = self.map_chunk(&chunk) {
                result.push_str(&mapped_value);
                 found_mapping = true;
                 prev_was_consonant = true;
                for _ in 0..3 { chars.next(); } // Drain the first 3 characters
                continue;
            }

            // Try to get the first 2 characters
            chunk = chars.clone().take(2).collect();
            if let Some(mapped_value) = self.map_chunk(&chunk) {
                result.push_str(&mapped_value);
                 found_mapping = true;
                  prev_was_consonant = true;
                for _ in 0..2 { chars.next(); } // Drain the first 2 characters
                continue;
            }
           
           if let Some(c) = chars.next() {
           
            let c_str = c.to_string(); 
           

            println!("Processing character: '{}'", c);

            if c == ' ' {
                result.push_str(&c_str);
                found_mapping = true;
                break;
            }
     

// explicit checkings for चन्द्रविन्दु ँ and शिरविन्दु ं
       if c == 'ṃ' {
             result.push('ं');
              found_mapping = true;
       }
             
         if c == 'ṅ' {
             result.push('ँ');
              found_mapping = true;
         }
            //println!("Character string: '{}'", c_str);

           if isbeginning && self.is_vowel(&c_str) {
               // println!("Beginning and current vowel: '{}'", c);

if let Some(independent_vowel) = self.reverse_mappings.iter().find_map(|((roman, vowel_type), nepali_list)| {
  if  roman == &c_str  &&  *vowel_type == VowelType::Independent   {
        nepali_list.iter().find_map(|nepali| Some(nepali.to_string()))
    } else {
        None
    }
}) {
    println!("Inependent vowel '{}'", independent_vowel);
    result.push_str(&independent_vowel);
    found_mapping = true;
    prev_was_consonant = false;
    isbeginning = false;
}
           
            
              }  
            



         
            
         else if prev_was_consonant && self.is_vowel(&c_str) {
                
              // println!("Prev consonant and current vowel: '{}'", c);

for ((roman, vowel_type), nepali_list) in &self.reverse_mappings {
    // Print debugging information
   // println!("Checking Roman: '{}',  c string '{}', Vowel Type: '{:?}'", roman, c_str, vowel_type);
    
    if roman == &c_str  &&  *vowel_type == VowelType::Dependent {
     // println!("nepali list {:?}",nepali_list);
      for dependent_vowel in nepali_list.iter() {
    println!("Matched Dependent Vowel: '{}'", dependent_vowel);
    result.push_str(dependent_vowel);
  }
  
        /*
          if let Some(dependent_vowel) = nepali_list.get(0) {
            println!("Matched Dependent Vowel: '{}'", dependent_vowel);
            result.push_str(dependent_vowel);
           
            break;
        } 
        *///
    }
}
              found_mapping = true;
             prev_was_consonant = false;
             
      
            } 
            
        //addition of halant ् after previous consonant is not implemented as it resulted many 'nuisance' 
       // than without it  
      /*           
            else if prev_was_consonant && self.is_consonant(&c_str) {
                println!("Prev consonant and current consonant: '{}'", c);
               
          if let Some(consonant) = self.reverse_mappings.iter().find_map(|((roman, vowel_type), nepali_list)| {
   if *vowel_type == VowelType::None &&  c_str.starts_with(roman) {
        nepali_list.iter().find_map(|nepali| Some(nepali.to_string()))
    } else {
        None
    }
}) {
    println!(" consonant '{}'", consonant);
    result.push('्');
    result.push_str(&consonant);
     prev_was_consonant = true;
    found_mapping = true;
}    
                
               
            
            
            } 
     */       
            else if self.is_consonant(&c_str) {
              //  println!("Current consonant: '{}'", c);
if let Some(consonant) = self.reverse_mappings.iter().find_map(|((roman, vowel_type), nepali_list)| {
  if c_str.starts_with(roman) && *vowel_type == VowelType::None{
        nepali_list.iter().find_map(|nepali| Some(nepali.to_string()))
    } else {
        None
    }
}) {
    println!(" consonant '{}'", consonant);
    result.push_str(&consonant);
    prev_was_consonant = true;
    found_mapping = true;
    
}
                
            }

            if !found_mapping {
                // Handling characters not in reverse_mappings
                println!("No mapping found for '{}'", c);
                result.push('?'); // Add ? when no mapping is found
            }
      if isbeginning  {
          isbeginning = false;
          
      } 
           
           }//big if 
           
        }//while
        
        result
    }


 fn map_chunk(&self, chunk: &str) -> Option<String> {
        for ((roman, vowel_type), nepali_list) in &self.reverse_mappings {
            if roman == chunk && matches!(vowel_type, VowelType::None) {
                if let Some(consonant) = nepali_list.first() {
                    println!("Matched chunk consonant: '{}'", consonant);
                    return Some(consonant.to_string());
                }
            }
        }
        None
    }








    
    
}//impl

