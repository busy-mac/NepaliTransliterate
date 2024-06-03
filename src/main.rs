use std::io::Result;
use std::fs::write;
use nepali_transliterator::NepaliTransliterator;

fn main() -> Result<()>{
  let transliterator = NepaliTransliterator::new();
  let nepali_text = "कलमले लेखेको आकाशद्वार पहाडि लालीगुराँस अक्षय स्मृति लाभांश श्री ज्ञान कोष क्षत्रपाटी काठमाण्डु ";

  let roman_text = transliterator.to_roman(nepali_text);
  println!("Roman: {}", roman_text);
  let output = format!("Roman: {}\n", roman_text);
  // If needed, converting back to Nepali for demonstration
  let nepali_back = transliterator.to_nepali(&roman_text);
  println!("Nepali: {}", nepali_back);

  let output = output + &format!("Nepali: {}\n", nepali_back);
  write("out.txt", output)?;
  Ok(())
}
