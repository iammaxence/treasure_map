use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error};

use crate::element::{self, Element, Orientation, RawAdventurer, RawMap, RawMountain, RawTreasure};

pub fn file_to_hashmap(file: File) -> Result<HashMap<String, Vec<Element>>, Error> {
    let separator = "-";
    let reader = BufReader::new(file);

    let mut hash_map: HashMap<String, Vec<Element>> = HashMap::new();

    for line in reader.lines() {
        let line: String = line?.split_whitespace().collect();
        let content: Vec<&str> = line.split(separator).collect();

        match content[0] {
            "C" => {
                let value = RawMap::new(content[1].parse().unwrap(), content[2].parse().unwrap());
                hash_map = insert_into_map(
                    hash_map,
                    content[0].to_owned(),
                    element::Element::RawMap(value),
                )
            }
            "M" => {
                let value =
                    RawMountain::new(content[1].parse().unwrap(), content[2].parse().unwrap());
                hash_map = insert_into_map(
                    hash_map,
                    content[0].to_owned(),
                    element::Element::RawMountain(value),
                )
            }
            "T" => {
                let value = RawTreasure::new(
                    content[1].parse().unwrap(),
                    content[2].parse().unwrap(),
                    content[3].parse().unwrap(),
                );
                hash_map = insert_into_map(
                    hash_map,
                    content[0].to_owned(),
                    element::Element::RawTreasure(value),
                )
            }
            "A" => {
                let orientation: Vec<char> = content[4].chars().collect();
                let value = RawAdventurer::new(
                    content[1].to_string(),
                    content[2].parse().unwrap(),
                    content[3].parse().unwrap(),
                    Orientation::from_char(orientation[0]),
                    content[5].to_string(),
                );
                hash_map = insert_into_map(
                    hash_map,
                    content[0].to_owned(),
                    element::Element::RawAdventurer(value),
                )
            }
            _ => println!("Ignore value : {}", content[0]),
        }
        println!("{}", line);
    }

    Ok(hash_map)
}

pub fn generate_map_file(
    hashmap: &HashMap<String, Vec<Element>>,
    filename: &str,
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    if let Some(raw_maps) = hashmap.get("C") {
        for element in raw_maps {
            if let Element::RawMap(raw_map) = element {
                writeln!(file, "C - {} - {}", raw_map.position.0, raw_map.position.1)?;
            }
        }
    }

    if let Some(raw_mountains) = hashmap.get("M") {
        for element in raw_mountains {
            if let Element::RawMountain(mountain) = element {
                writeln!(
                    file,
                    "M - {} - {}",
                    mountain.position.0, mountain.position.1
                )?;
            }
        }
    }

    if let Some(raw_treasures) = hashmap.get("T") {
        for element in raw_treasures {
            if let Element::RawTreasure(treasure) = element {
                writeln!(
                    file,
                    "T - {} - {} - {}",
                    treasure.position.0, treasure.position.1, treasure.nb_treasure
                )?;
            }
        }
    }

    if let Some(raw_adventurers) = hashmap.get("A") {
        for element in raw_adventurers {
            if let Element::RawAdventurer(adventurer) = element {
                writeln!(
                    file,
                    "A - {} - {} - {} - {} - {}",
                    adventurer.name,
                    adventurer.position.0,
                    adventurer.position.1,
                    adventurer.get_orientation_to_string(),
                    adventurer.treasure
                )?;
            }
        }
    }

    Ok(())
}

fn insert_into_map(
    hash_map: HashMap<String, Vec<Element>>,
    key: String,
    value: Element,
) -> HashMap<String, Vec<Element>> {
    let mut new_hash_map = hash_map.clone();
    let mut new_vec = vec![value];

    if new_hash_map.contains_key(&key) {
        let current_vec = new_hash_map.get_mut(&key).unwrap();
        current_vec.append(&mut new_vec);
        new_vec = current_vec.to_vec();
    }

    new_hash_map.insert(key, new_vec);

    new_hash_map
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn should_convert_file_to_hashmap() {
        // Given
        let mut temp_file = NamedTempFile::new().expect("Failed to create tempfile");
        writeln!(temp_file, "C - 1 - 2").expect("Failed to write to tempfile");
        writeln!(temp_file, "M - 2 -3").expect("Failed to write to tempfile");
        writeln!(temp_file, "M-1-1").expect("Failed to write to tempfile");
        writeln!(temp_file, "T-2-2-1").expect("Failed to write to tempfile");
        writeln!(temp_file, "A-Lara-0-3-S-AADADA").expect("Failed to write to tempfile");
        temp_file.flush().expect("Failed to flush tempfile");
        let file = temp_file.reopen().expect("Failed to reopen tempfile");

        // When
        let result = file_to_hashmap(file).expect("Failed to read file and create hashmap");

        // Then
        let mut expected: HashMap<String, Vec<Element>> = HashMap::new();
        expected.insert(
            "C".to_string(),
            vec![element::Element::RawMap(RawMap::new(1, 2))],
        );
        expected.insert(
            "M".to_string(),
            vec![
                element::Element::RawMountain(RawMountain::new(2, 3)),
                element::Element::RawMountain(RawMountain::new(1, 1)),
            ],
        );
        expected.insert(
            "T".to_string(),
            vec![element::Element::RawTreasure(RawTreasure::new(2, 2, 1))],
        );
        expected.insert(
            "A".to_string(),
            vec![element::Element::RawAdventurer(RawAdventurer::new(
                "Lara".to_string(),
                0,
                3,
                Orientation::South,
                "AADADA".to_string(),
            ))],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_map_file() -> io::Result<()> {
        // Given
        let mut hashmap: HashMap<String, Vec<Element>> = HashMap::new();
        hashmap.insert(
            "C".to_string(),
            vec![element::Element::RawMap(RawMap::new(4, 4))],
        );
        hashmap.insert(
            "M".to_string(),
            vec![
                Element::RawMountain(RawMountain { position: (0, 1) }),
                Element::RawMountain(RawMountain { position: (1, 1) }),
            ],
        );
        hashmap.insert(
            "T".to_string(),
            vec![Element::RawTreasure(RawTreasure::new(1, 2, 2))],
        );
        hashmap.insert(
            "A".to_string(),
            vec![Element::RawAdventurer(RawAdventurer::new(
                "Lara".to_string(),
                0,
                0,
                Orientation::South,
                "GAADADAAGADA".to_string(),
            ))],
        );

        // When
        generate_map_file(&hashmap, "test_map.txt")?;

        // Then
        let expected_content =
            "C - 4 - 4\nM - 0 - 1\nM - 1 - 1\nT - 1 - 2 - 2\nA - Lara - 0 - 0 - S - 0\n";
        let actual_content = std::fs::read_to_string("test_map.txt")?;
        assert_eq!(actual_content, expected_content);

        std::fs::remove_file("test_map.txt")?;

        Ok(())
    }
}
