use std::collections::HashMap;

use crate::element::{Element, Empty, RawAdventurer, RawMap};

pub struct Map {
    pub rows: usize,
    pub cols: usize,
    pub elements: Vec<Vec<Element>>,
}

impl Map {
    pub fn new(rows: usize, cols: usize, content: HashMap<String, Vec<Element>>) -> Map {
        let mut new_vec: Vec<Vec<Element>> = (0..rows)
            .map(|row| {
                (0..cols)
                    .map(|col| Element::Empty(Empty::new(row, col)))
                    .collect()
            })
            .collect();

        Self::fill_with_data(&mut new_vec, &content);

        Map {
            rows,
            cols,
            elements: new_vec,
        }
    }

    pub fn to_hashmap(&self, adventurers: Vec<RawAdventurer>) -> HashMap<String, Vec<Element>> {
        let mut hashmap: HashMap<String, Vec<Element>> = HashMap::new();

        for (_, row) in self.elements.iter().enumerate() {
            for (_, element) in row.iter().enumerate() {
                let key = match element {
                    Element::RawMountain(_) => "M".to_string(),
                    Element::RawTreasure(_) => "T".to_string(),
                    _ => continue,
                };
                hashmap
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            }
        }

        hashmap
            .entry("C".to_string())
            .or_insert_with(Vec::new)
            .push(Element::RawMap(RawMap::new(self.rows, self.cols)));

        hashmap.insert(
            "A".to_string(),
            adventurers
                .into_iter()
                .map(Element::RawAdventurer)
                .collect(),
        );

        hashmap
    }

    pub fn update_position(&mut self, x: usize, y: usize) {
        if let Some(element) = self.elements.get_mut(x).and_then(|row| row.get_mut(y)) {
            match element {
                Element::RawTreasure(treasure_element) => {
                    treasure_element.nb_treasure -= 1;
                }
                _ => (),
            }
        }
    }

    pub fn print_map(map_data: &Vec<Vec<Element>>) {
        for row in map_data {
            for element in row {
                print!("{:?} ", element);
            }
            println!();
        }
    }

    fn fill_with_data(my_map: &mut Vec<Vec<Element>>, content: &HashMap<String, Vec<Element>>) {
        for (_, value) in content.iter() {
            for data in value.iter() {
                match data {
                    Element::RawMountain(_) | Element::RawTreasure(_) => {
                        let coordinates = data.position().unwrap();
                        my_map[coordinates.0][coordinates.1] = data.clone();
                    }
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::element::{Orientation, RawAdventurer, RawMap, RawMountain, RawTreasure};

    use super::*;

    const ROW_SIZE_MAP: usize = 3;
    const COL_SIZE_MAP: usize = 4;

    /* MOCK INIT */

    fn init_mock_map() -> Map {
        let mut hash_map: HashMap<String, Vec<Element>> = HashMap::new();
        hash_map.insert(
            "M".to_string(),
            vec![
                Element::RawMap(RawMap::new(4, 4)),
                Element::RawAdventurer(RawAdventurer::new(
                    "Lara".to_string(),
                    4,
                    4,
                    Orientation::South,
                    "AA".to_string(),
                )),
                Element::RawTreasure(RawTreasure::new(0, 1, 2)),
                Element::RawMountain(RawMountain::new(2, 3)),
                Element::RawMountain(RawMountain::new(1, 1)),
            ],
        );

        Map::new(ROW_SIZE_MAP, COL_SIZE_MAP, hash_map)
    }

    /* SHOULD INIT MAP */

    #[test]
    fn should_create_new_map() {
        // Given + When
        let my_map = init_mock_map();

        // Then
        let mut expect: Vec<Vec<Element>> = (0..ROW_SIZE_MAP)
            .map(|row| {
                (0..COL_SIZE_MAP)
                    .map(|col| Element::Empty(Empty::new(row, col)))
                    .collect()
            })
            .collect();
        expect[0][1] = Element::RawTreasure(RawTreasure::new(0, 1, 2));
        expect[2][3] = Element::RawMountain(RawMountain::new(2, 3));
        expect[1][1] = Element::RawMountain(RawMountain::new(1, 1));

        assert_eq!(my_map.elements, expect);
    }

    /* SHOULD UPDATE POSITION */

    #[test]
    fn should_update_nb_treasure_when_element_at_position_x_y_is_treasure() {
        // Given
        let mut my_map = init_mock_map();

        // When
        Map::update_position(&mut my_map, 0, 1);

        // Then
        let mut expect: Vec<Vec<Element>> = init_mock_map().elements;
        expect[0][1] = Element::RawTreasure(RawTreasure::new(0, 1, 1));

        assert_eq!(my_map.elements, expect);
    }

    /* SHOULD CONVERT TO HASHMAP */

    #[test]
    fn should_map_to_hash_map() {
        // Given
        let my_map = init_mock_map();
        let adventurers = vec![RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::East,
            "A".to_string(),
        )];

        // When
        let result_hashmap = my_map.to_hashmap(adventurers);

        // Then
        let mut expected: HashMap<String, Vec<Element>> = HashMap::new();
        expected.insert(
            "C".to_string(),
            vec![Element::RawMap(RawMap::new(ROW_SIZE_MAP, COL_SIZE_MAP))],
        );
        expected.insert(
            "M".to_string(),
            vec![
                Element::RawMountain(RawMountain::new(1, 1)),
                Element::RawMountain(RawMountain::new(2, 3)),
            ],
        );
        expected.insert(
            "T".to_string(),
            vec![Element::RawTreasure(RawTreasure::new(0, 1, 2))],
        );
        expected.insert(
            "A".to_string(),
            vec![Element::RawAdventurer(RawAdventurer::new(
                "Lara".to_string(),
                0,
                0,
                Orientation::East,
                "A".to_string(),
            ))],
        );

        assert_eq!(expected, result_hashmap);
    }

    /* SHOULD PANIC */

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn should_panic_when_data_filled_with_an_incorrect_position() {
        // Given
        let mut hash_map: HashMap<String, Vec<Element>> = HashMap::new();
        hash_map.insert(
            "M".to_string(),
            vec![
                Element::RawMountain(RawMountain::new(5, 5)),
                Element::RawMountain(RawMountain::new(1, 1)),
            ],
        );

        // When + Then
        Map::new(3, 4, hash_map);
    }
}
