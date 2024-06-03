use std::collections::HashMap;

pub struct NepaliTransliterator {
    mappings: HashMap<String, (String, VowelType)>,
    reverse_mappings: HashMap<String, String>,
}

#[derive(Clone)]
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
            ('ञ', ("ña", VowelType::None)),
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
            ('ि', ("i", VowelType::Dependent)),
            ('ी', ("ī", VowelType::Dependent)),
            ('ो', ("o", VowelType::Dependent)),
            ('ु', ("u", VowelType::Dependent)),
            ('ू', ("ū", VowelType::Dependent)),
            ('े', ("e", VowelType::Dependent)),
            ('ा', ("ā", VowelType::Dependent)),
            ('ृ', ("ṛ", VowelType::Dependent)),
            ('ः', ("ḥ", VowelType::Dependent)),
        ];

        for (nepali_char, (roman_char, vowel_type)) in map_data {
            mappings.insert(
                nepali_char.to_string(),
                (roman_char.to_string(), vowel_type),
            );
        }

        let mut reverse_mappings = HashMap::new();
        for (nepali, (roman, _)) in &mappings {
            reverse_mappings.insert(roman.clone(), nepali.clone());
        }
      
        NepaliTransliterator {
            mappings,
            reverse_mappings,
        }
        
      
    }

    // Function to check if a character is a consonant (modify based on your mapping structure)
    fn is_consonant(&self, ch: &str) -> bool {
        matches!(self.mappings.get(ch), Some((_, VowelType::None)))
    }

    fn is_vowel(&self, ch: &str) -> bool {
        matches!(self.reverse_mappings.get(ch), Some(nepali_char) if matches!(self.mappings.get(nepali_char), Some((_, VowelType::Independent | VowelType::Dependent))))
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

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                if !buffer.is_empty() {
                    result.push_str(&self.process_buffer(&buffer));
                    buffer.clear();
                }
                result.push(c); // Add whitespace character directly to result
            } else {
                buffer.push(c);
                if self.is_vowel(&buffer) || chars.peek().map_or(false, |c| c.is_whitespace() || self.is_vowel(&c.to_string())) {
                    result.push_str(&self.process_buffer(&buffer));
                    buffer.clear();
                }
            }
        }

        if !buffer.is_empty() {
            result.push_str(&self.process_buffer(&buffer));
        }

        result
    }

    fn process_buffer(&self, buffer: &str) -> String {
        let mut result = String::new();
        let mut temp = String::new();
        let mut prev_char_is_consonant = false;

        for c in buffer.chars() {
            let input_char = c.to_string();

            if self.is_vowel(&input_char) && result.is_empty() {
                // Vowel at the beginning
                if let Some(nepali_char) = self.reverse_mappings.get(&input_char) {
                    result.push_str(nepali_char);
                } else {
                    println!("Character '{}' not found in reverse_mappings", input_char);
                    result.push_str(&input_char);
                }
                continue;
            }

            temp.push(c);

            if self.is_vowel(&temp) || c.is_whitespace() {
                if let Some(nepali_char) = self.reverse_mappings.get(&temp) {
                    if prev_char_is_consonant {
                        result.push('्'); // Add halant
                    }
                    result.push_str(nepali_char);
                } else {
                    let mut chars = temp.chars().peekable();
                    while let Some(ch) = chars.next() {
                        let input_char = ch.to_string();
                        if let Some(nepali_char) = self.reverse_mappings.get(&input_char) {
                            if prev_char_is_consonant {
                                result.push('्'); // Add halant
                            }
                            result.push_str(nepali_char);
                            prev_char_is_consonant = !self.is_vowel(&input_char);
                        } else {
                            println!("Character '{}' not found in reverse_mappings", input_char);
                            result.push_str(&input_char);
                        }
                    }
                }
                temp.clear();
            } else {
                prev_char_is_consonant = true;
            }
        }

        if !temp.is_empty() {
            if let Some(nepali_char) = self.reverse_mappings.get(&temp) {
                if prev_char_is_consonant {
                    result.push('्'); // Add halant
                }
                result.push_str(nepali_char);
            } else {
                for ch in temp.chars() {
                    let input_char = ch.to_string();
                    if let Some(nepali_char) = self.reverse_mappings.get(&input_char) {
                        if prev_char_is_consonant {
                            result.push('्'); // Add halant
                        }
                        result.push_str(nepali_char);
                        prev_char_is_consonant = !self.is_vowel(&input_char);
                    } else {
                        println!("Character '{}' not found in reverse_mappings", input_char);
                        result.push_str(&input_char);
                    }
                }
            }
        }

        result
    }



    
    
}//impl

