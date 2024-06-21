//बिजी७७<bandesh@gmail.com>

use std::io;
use std::fs;
use clap::{App, Arg};
use NepaliTransliterate::NepaliTransliterator;

fn main() -> Result<(), io::Error> {
    
    println!("Usage: ./nepalitransiterate -h \n to roman:./nepali-tranliterate -r \'nepali text\' -o output_file(optional) \n  to nepali:./nepali-tranliterate -n \'roman text\' -o output_file(optional) \n ");
    
    let matches = App::new("Nepali Transliterator")
     .version("1.0")
     .author("बिजी७७")
     .about("Transliterate Nepali text to Roman and vice versa")
     .arg(Arg::with_name("input")
          .help("Input text to transliterate")
          .required(false)
          .index(1))
     .arg(Arg::with_name("toroman")
          .help("Transliterate to Roman")
          .short("r")
          .long("toroman"))
     .arg(Arg::with_name("tonepali")
          .help("Transliterate to Nepali")
          .short("n")
          .long("tonepali"))
     .arg(Arg::with_name("output")
          .help("Output file")
          .short("o")
          .long("output")
          .takes_value(true))
     .get_matches();

    if matches.is_present("help") {
        println!("Usage:
./nepali-tranliterate -h
to roman:./nepali-tranliterate -r \'nepali text\' -o output_file(optional)
to nepali:./nepali-tranliterate -n \'roman text\' -o output_file(optional) ");
        return Ok(());
    }

    let input_text = if let Some(input) = matches.value_of("input") {
        input
    } else {
        "कलमले लेखेको आकाशद्वार पहाडि लालीगुराँस अक्षय स्मृति लाभांश श्री ज्ञान कोष क्षत्रपाटी काठमाण्डु "
    };

    let output_file = matches.value_of("output");

    let transliterator = NepaliTransliterator::new();

    let output = match (matches.is_present("toroman"), matches.is_present("tonepali")) {
        (true, false) => {
            let roman_text = transliterator.to_roman(input_text);
            format!("Roman: {}\n", roman_text)
        }
        (false, true) => {
            let nepali_text = transliterator.to_nepali(input_text);
            format!("Nepali: {}\n", nepali_text)
        }
        _ => {
            let roman_text = transliterator.to_roman(input_text);
            let nepali_back = transliterator.to_nepali(&roman_text);
            format!("Roman: {}\nNepali: {}\n", roman_text, nepali_back)
        }
    };

    if let Some(file) = output_file {
        fs::write(file, output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}
