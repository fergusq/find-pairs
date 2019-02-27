use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};

fn main() {
    let app = App::new("find-pairs")
        .version("0.1.0")
        .author("Iikka Hauhio <iikka.hauhio@helsinki.fi>")
        .about("Find grammatical pairs in CONLL-U data")
        .arg(Arg::with_name("data").required(true))
        .arg(Arg::with_name("rel").required(true));
    let matches = app.get_matches();
    let datafile = matches.value_of("data").unwrap();
    let rel = matches.value_of("rel").unwrap();

    let reader = BufReader::new(File::open(datafile).expect("data file could not be opened"));
    let data = conllu_rs::parse_conllu(reader.lines().flatten());

    for sentence in data {
        for word in &sentence {
            if word.deprel == rel {
                println!("{} {}", word.form, sentence[word.head-1].form);
            }
        }
    }
}
