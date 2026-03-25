use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.fst");
    let stopwords_path = Path::new(&out_dir).join("stopwords.fst");

    build_fst("data/words.txt", &dest_path);
    build_fst("data/stopwords.txt", &stopwords_path);

    println!("cargo:rerun-if-changed=data/words.txt");
    println!("cargo:rerun-if-changed=data/stopwords.txt");
    println!("cargo:rerun-if-changed=build.rs");
}

fn build_fst(src: &str, dest: &Path) {
    let file = File::open(src).expect("could not open source words file");
    let reader = BufReader::new(file);
    let mut words: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    words.sort();
    words.dedup();

    let dest_file = File::create(dest).expect("could not create fst file");
    let mut build = fst::SetBuilder::new(BufWriter::new(dest_file)).unwrap();
    for word in words {
        build.insert(word).unwrap();
    }
    build.finish().unwrap();
}
