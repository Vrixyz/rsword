use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::BufRead,
};

pub struct PossibleWords {
    pub words_tree: WordTree,
}

#[derive(Default, Hash, Eq, PartialEq)]
pub struct WordTree {
    pub next: BTreeMap<char, Box<WordTree>>,
    pub can_be_last_letter: bool,
}

impl WordTree {
    fn _visit<F>(&self, parents: Vec<char>, callback: &mut F)
    where
        F: FnMut(&Vec<char>),
    {
        if self.can_be_last_letter {
            callback(&parents);
        }
        for kv in self.next.iter() {
            let mut new_parents = parents.clone();
            dbg!(kv.0);
            new_parents.push(*kv.0);
            kv.1._visit(new_parents, callback);
        }
    }

    pub fn visit<F>(&self, callback: &mut F)
    where
        F: FnMut(&Vec<char>),
    {
        self._visit(vec![], callback)
    }
}

pub fn load_from<B: BufRead>(reader: B) -> PossibleWords {
    let mut tree_root = WordTree::default();
    let lines = reader.lines();
    for line in lines {
        let Ok(line) = line else {
        continue;
       };
        let mut tree_ref = &mut tree_root.next;
        let length = line.len();
        for (i, letter) in line.chars().enumerate() {
            if let std::collections::btree_map::Entry::Vacant(e) = tree_ref.entry(letter) {
                e.insert(Box::<WordTree>::default());
            };
            if i == length - 1 {
                tree_ref.get_mut(&letter).unwrap().can_be_last_letter = true;
            }
            tree_ref = &mut tree_ref.get_mut(&letter).unwrap().next;
        }
    }
    PossibleWords {
        words_tree: tree_root,
    }
}

mod test {

    #[test]
    fn parse() {
        let words = r#"hello
world
hell
worms
fantastic
"#;
        let tree_root = super::load_from(words.as_bytes());
        let mut count = 0;
        tree_root.words_tree.visit(&mut |_| {
            count += 1;
        });
        assert_eq!(count, 5);
    }
}
