mod word_table;
mod word_tree;

use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
};

use word_tree::load_from;

fn main() {
    let f = File::open("assets/Words.en.txt").expect("Could not read file.");
    let reader = BufReader::new(f);
    let tree_root = load_from(reader);

    let mut buffer_writer = BufWriter::new(io::stdout().lock());

    tree_root.words_tree.visit(&mut |word| {
        let _ = buffer_writer
            .write(word.iter().collect::<String>().as_bytes())
            .unwrap();
        let _ = buffer_writer.write(&[b'\n']).unwrap();
    });
    buffer_writer.flush().unwrap();
}
