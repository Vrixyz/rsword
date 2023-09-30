use glam::IVec2;
use std::{
    collections::{HashMap, VecDeque},
    vec,
};

pub struct Tile {
    pub team: usize,
    pub character: char,
}

pub struct Table {
    pub tiles: HashMap<IVec2, Tile>,
}

impl Table {
    pub fn get_vertical_words(&self) -> Vec<(IVec2, Vec<&Tile>)> {
        let mut key_by_x = self
            .tiles
            .keys()
            .into_iter()
            .copied()
            .collect::<Vec<IVec2>>();
        key_by_x.reverse();
        key_by_x.sort_unstable_by_key(|v| -v.x);
        let mut vertical_words = vec![];
        let Some(mut previous_pos) = key_by_x.pop() else {
            return vertical_words;
        };
        dbg!("first word at: ", previous_pos);
        let mut current_word = (previous_pos, vec![&self.tiles[&previous_pos]]);
        while let Some(new_pos) = key_by_x.pop() {
            let mut new_word = false;
            if new_pos.x != previous_pos.x {
                dbg!("new vertical line to x = ", new_pos.x);
                new_word = true;
            }
            if new_pos.y != previous_pos.y + 1 {
                dbg!("new vertical line to y = ", new_pos.y);
                new_word = true;
            }
            if new_word {
                vertical_words.push(current_word);
                current_word = (new_pos, vec![&self.tiles[&new_pos]]);
            } else {
                current_word.1.push(&self.tiles[&new_pos]);
            }
            previous_pos = new_pos;
        }
        vertical_words.push(current_word);
        vertical_words
    }
    pub fn get_horizontal_words(&self) -> Vec<(IVec2, Vec<&Tile>)> {
        let mut key_by_y = self
            .tiles
            .keys()
            .into_iter()
            .copied()
            .collect::<Vec<IVec2>>();
        key_by_y.reverse();
        key_by_y.sort_unstable_by_key(|v| -v.y);
        let mut horizontal_words = vec![];
        let Some(mut previous_pos) = key_by_y.pop() else {
            return horizontal_words;
        };
        dbg!("first word at: ", previous_pos);
        let mut current_word = (previous_pos, vec![&self.tiles[&previous_pos]]);
        while let Some(new_pos) = key_by_y.pop() {
            let mut new_word = false;
            if new_pos.y != previous_pos.y {
                dbg!("new vertical line to y = ", new_pos.y);
                new_word = true;
            }
            if new_pos.x != previous_pos.x + 1 {
                dbg!("new vertical line to x = ", new_pos.x);
                new_word = true;
            }
            if new_word {
                horizontal_words.push(current_word);
                current_word = (new_pos, vec![&self.tiles[&new_pos]]);
            } else {
                current_word.1.push(&self.tiles[&new_pos]);
            }
            previous_pos = new_pos;
        }
        horizontal_words.push(current_word);
        horizontal_words
    }

    pub fn get_words(&self) -> (Vec<(IVec2, Vec<&Tile>)>, Vec<(IVec2, Vec<&Tile>)>) {
        (self.get_vertical_words(), self.get_horizontal_words())
    }
}

mod tests {
    use std::collections::HashMap;

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
            ]),
        };
        let words = table.get_words();
        for horizontal_word in words.0 {
            let word = horizontal_word
                .1
                .iter()
                .map(|tile| tile.character)
                .collect::<String>();
            dbg!(word);
        }
    }
}
