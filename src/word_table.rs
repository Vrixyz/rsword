use glam::IVec2;
use std::{collections::HashMap, vec};

#[derive(Clone)]
pub struct Tile {
    pub team: usize,
    pub character: char,
}

#[derive(Default)]
pub struct Table {
    pub tiles: HashMap<IVec2, Tile>,
}

impl Table {
    pub fn get_vertical_words(&self) -> Vec<WordOnTable> {
        let mut ordered_keys = self.tiles.keys().copied().collect::<Vec<IVec2>>();
        ordered_keys.reverse();
        ordered_keys.sort_unstable_by_key(|v| -v.y);
        let mut horizontal_words = vec![];
        while let Some(mut moving_letter_pos) = ordered_keys.pop() {
            let mut current_word = WordOnTable {
                position: moving_letter_pos,
                tiles: vec![],
            };
            while let Some(next_letter) = self.tiles.get(&moving_letter_pos) {
                current_word.tiles.push(next_letter);
                moving_letter_pos += IVec2::Y;
            }
            let word_range = current_word.position.y
                ..=(current_word.position.y + current_word.tiles.len() as i32);
            ordered_keys.retain(|e| e.x != current_word.position.x || !word_range.contains(&e.y));
            horizontal_words.push(current_word);
        }
        horizontal_words
    }
    pub fn get_horizontal_words(&self) -> Vec<WordOnTable> {
        let mut ordered_keys = self.tiles.keys().copied().collect::<Vec<IVec2>>();
        ordered_keys.reverse();
        ordered_keys.sort_unstable_by_key(|v| -v.x);
        let mut horizontal_words = vec![];
        while let Some(mut moving_letter_pos) = ordered_keys.pop() {
            let mut current_word = WordOnTable {
                position: moving_letter_pos,
                tiles: vec![],
            };
            while let Some(next_letter) = self.tiles.get(&moving_letter_pos) {
                current_word.tiles.push(next_letter);
                moving_letter_pos += IVec2::X;
            }
            let word_range = current_word.position.x
                ..=(current_word.position.x + current_word.tiles.len() as i32);
            ordered_keys.retain(|e| e.y != current_word.position.y || !word_range.contains(&e.x));
            horizontal_words.push(current_word);
        }
        horizontal_words
    }

    pub fn get_words(&self) -> TableWordsList {
        TableWordsList {
            horizontal: self.get_horizontal_words(),
            vertical: self.get_vertical_words(),
        }
    }
}

pub struct WordOnTable<'a> {
    pub position: IVec2,
    pub tiles: Vec<&'a Tile>,
}

impl<'a> WordOnTable<'a> {
    pub fn get_word(&'a self) -> String {
        return self
            .tiles
            .iter()
            .map(|tile| tile.character)
            .collect::<String>();
    }
}

pub struct TableWordsList<'a> {
    pub horizontal: Vec<WordOnTable<'a>>,
    pub vertical: Vec<WordOnTable<'a>>,
}

mod tests {
    use std::collections::HashMap;

    use glam::IVec2;

    use super::{Table, Tile};

    #[test]
    fn table_get_words() {
        let table = Table {
            tiles: HashMap::from([
                (
                    (0, 0).into(),
                    Tile {
                        team: 0,
                        character: 'h',
                    },
                ),
                (
                    (0, 1).into(),
                    Tile {
                        team: 0,
                        character: 'e',
                    },
                ),
                (
                    (0, 2).into(),
                    Tile {
                        team: 0,
                        character: 'y',
                    },
                ),
                (
                    (1, 2).into(),
                    Tile {
                        team: 0,
                        character: 'o',
                    },
                ),
                (
                    (2, 2).into(),
                    Tile {
                        team: 0,
                        character: 'u',
                    },
                ),
                (
                    (4, 2).into(),
                    Tile {
                        team: 0,
                        character: 'a',
                    },
                ),
            ]),
        };
        let words = table.get_words();
        assert!(words.horizontal.iter().any(|t| t.get_word() == "e"));
        assert!(words
            .horizontal
            .iter()
            .any(|t| t.get_word() == "you" && t.position == IVec2::new(0, 2)));
        assert!(words.horizontal.iter().any(|t| t.get_word() == "a"));
        assert!(words.horizontal.iter().any(|t| t.get_word() == "e"));
        assert_eq!(words.horizontal.len(), 4);

        assert!(words.vertical.iter().any(|t| t.get_word() == "u"));
        assert!(words
            .vertical
            .iter()
            .any(|t| t.get_word() == "hey" && t.position == IVec2::new(0, 0)));
        assert!(words.vertical.iter().any(|t| t.get_word() == "a"));
        assert!(words.vertical.iter().any(|t| t.get_word() == "o"));
        assert_eq!(words.vertical.len(), 4);
    }
}
