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
        let mut prev_char_info: Option<(String, VowelType)> = None;
        let mut buffer = String::new();

        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                if !buffer.is_empty() {
                    result.push_str(&self.process_buffer(&buffer, &mut prev_char_info));
                    buffer.clear();
                }
                result.push(c);
                prev_char_info = None;
            } else {
                buffer.push(c);
                if chars.peek().is_none() || chars.peek().unwrap().is_whitespace() {
                    result.push_str(&self.process_buffer(&buffer, &mut prev_char_info));
                    buffer.clear();
                }
            }
        }

        if !buffer.is_empty() {
            result.push_str(&self.process_buffer(&buffer, &mut prev_char_info));
        }

        result
    }

    fn process_buffer(&self, buffer: &str, prev_char_info: &mut Option<(String, VowelType)>) -> String {
        let mut result = String::new();
        let mut is_first_letter = true;

        for c in buffer.chars() {
            let input_char = c.to_string();

            let nepali_char = match self.reverse_mappings.get(&input_char) {
                Some(nepali_char) => nepali_char.clone(),
                None => {
                    println!("Character '{}' not found in reverse_mappings", input_char);
                    input_char.clone()
                }
            };

            match (is_first_letter, self.mappings.get(&nepali_char)) {
                (true, Some((_, VowelType::Independent))) => {
                    result.push_str(&nepali_char);
                }
                (true, Some((_, VowelType::None))) => {
                    result.push_str(&nepali_char);
                }
                (false, Some((_, VowelType::None))) => {
                    if self.is_consonant(&input_char) {
                        if let Some((_, VowelType::None)) = prev_char_info {
                            result.push('्');
                        }
                        result.push_str(&nepali_char);
                    } else {
                        result.push_str(&nepali_char);
                    }
                }
                (false, Some((_, VowelType::Dependent))) => {
                    if let Some((prev_char, _)) = prev_char_info {
                        result.push_str(&nepali_char);
                    } else {
                        println!("Dependent vowel '{}' without preceding consonant", input_char);
                        result.push_str(&nepali_char);
                    }
                }
                _ => {
                    println!("Mapping not found for character '{}'", input_char);
                    result.push_str(&input_char);
                }
            }

            *prev_char_info = self.mappings.get(&nepali_char).cloned();
            is_first_letter = false;
        }

        result
    }
    

    /*
          pub fn to_nepali(&self, input: &str) -> String {
            let mut result = String::new();
            let mut prev_was_consonant = false;
            let mut prev_was_vowel = false;

            for c in input.chars() {
              if c.is_whitespace() {
                result.push(c); // Add whitespace character directly to result
              } else {
                let nepali_char = self.mappings.get(&c.to_string());  // Lookup in hashmap (no case conversion)
                if let Some(nepali_char) = nepali_char {
                  // Handle mapped characters
                  result.push_str(nepali_char);
                  prev_was_consonant = nepali_char.ends_with('्');  // Check for halanta in mapping
                  prev_was_vowel = nepali_char.chars().all(|x| x.is_ascii_alphabetic() && !x.is_uppercase()); // Check if all chars are lowercase letters
                } else {
                  // Handle unmapped characters
                  result.push(c);
                }

                // Halanta insertion logic (consider both mapped and unmapped characters)
                if c.is_alphabetic() && prev_was_consonant && !prev_was_vowel && !c.is_uppercase() {
                  // Only insert halanta if the current character is a lowercase letter
                  result.push('्');  // Insert halanta between consonants (except after vowels)
                }
                prev_was_consonant = c.is_alphabetic();
                prev_was_vowel = c.is_ascii_alphabetic() && !c.is_uppercase();  // Track vowel for halanta logic
              }
            }
            result
          }

    **/
}

/*
use std::collections::HashMap;

pub struct Transliterator {
    nepali_to_roman: HashMap<char, &'static str>,
    roman_to_nepali: HashMap<&'static str, char>,
}

impl Transliterator {
    pub fn new() -> Self {
        let mut nepali_to_roman = HashMap::new();
        let mut roman_to_nepali = HashMap::new();

        let mappings = vec![
            ('अ', "a"), ('आ', "ā"), ('इ', "i"), ('ई', "ī"), ('उ', "u"),
            ('ऊ', "ū"), ('ए', "e"), ('ऐ', "ai"), ('ओ', "o"), ('औ', "au"),
            ('क', "k"), ('ख', "kh"), ('ग', "g"), ('घ', "gh"), ('च', "c"),
            ('छ', "ch"), ('ज', "j"), ('झ', "jh"), ('ट', "ṭ"), ('ठ', "ṭh"),
            ('ड', "ḍ"), ('ढ', "ḍh"), ('ण', "ṇ"), ('त', "t"), ('थ', "th"),
            ('द', "d"), ('ध', "dh"), ('न', "n"), ('प', "p"), ('फ', "ph"),
            ('ब', "b"), ('भ', "bh"), ('म', "m"), ('य', "y"), ('र', "r"),
            ('ल', "l"), ('व', "v"), ('श', "ś"), ('ष', "ṣ"), ('स', "s"),
            ('ह', "h"),
        ];

        for (nep, rom) in mappings {
            nepali_to_roman.insert(nep, rom);
            roman_to_nepali.insert(rom, nep);
        }

        Self {
            nepali_to_roman,
            roman_to_nepali,
        }
    }


    pub fn to_roman(&self, input: &str) -> String {
        input.chars().map(|c| {

                    self.nepali_to_roman.get(&c).cloned().unwrap_or_else(|| Box::leak(Box::new(c.to_string())))
                }).collect()
            }

    pub fn to_nepali(&self, input: &str) -> String {
        // This implementation needs to handle multi-char mappings properly
        let mut result = String::new();
        let mut i = 0;
        while i < input.len() {
          // Iterate through characters
          for c in input.chars().skip(i) {
            let candidate = c.to_string(); // Convert character to string for HashMap lookup
            if let Some(&nepali) = self.roman_to_nepali.get(candidate.as_str()) {
              result.push(nepali);
              i += candidate.len();
              break; // Exit inner loop if mapping found
            }
          }
          // If no mapping found for single character, append it as-is (assuming not multi-char)
          if i == input.len() {
            result.push(input.chars().nth(i).unwrap());
          }
          i += 1;
        }
        result
    }
}



*/
