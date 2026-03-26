use fst::SetBuilder;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.fst");
    let wtr = File::create(dest_path).unwrap();
    let mut builder = SetBuilder::new(wtr).unwrap();

    let file = File::open("data/words.txt").unwrap();
    let reader = BufReader::new(file);

    let mut words: Vec<String> = reader.lines().map(|l| l.unwrap().to_lowercase()).collect();
    words.sort();
    words.dedup();

    for word in words {
        builder.insert(word).unwrap();
    }
    builder.finish().unwrap();

    // stopwords
    let dest_path_stop = Path::new(&out_dir).join("stopwords.fst");
    let wtr_stop = File::create(dest_path_stop).unwrap();
    let mut builder_stop = SetBuilder::new(wtr_stop).unwrap();
    let file_stop = File::open("data/stopwords.txt").unwrap();
    let reader_stop = BufReader::new(file_stop);
    let mut words_stop: Vec<String> = reader_stop
        .lines()
        .map(|l| l.unwrap().to_lowercase())
        .collect();
    words_stop.sort();
    words_stop.dedup();
    for word in words_stop {
        builder_stop.insert(word).unwrap();
    }
    builder_stop.finish().unwrap();

    // javanese
    let dest_path_jv = Path::new(&out_dir).join("javanese.fst");
    let wtr_jv = File::create(dest_path_jv).unwrap();
    let mut builder_jv = SetBuilder::new(wtr_jv).unwrap();
    let file_jv = File::open("data/javanese_words.txt").unwrap();
    let reader_jv = BufReader::new(file_jv);
    let mut words_jv: Vec<String> = reader_jv
        .lines()
        .map(|l| l.unwrap().to_lowercase())
        .collect();
    words_jv.sort();
    words_jv.dedup();
    for word in words_jv {
        builder_jv.insert(word).unwrap();
    }
    builder_jv.finish().unwrap();
}
