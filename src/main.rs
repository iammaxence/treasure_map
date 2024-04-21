use std::{collections::HashMap, fs::File, io};

use crate::{
    element::{Element, Orientation, RawAdventurer, RawMap},
    file::generate_map_file,
    map::Map,
};

mod element;
mod file;
mod map;

static INPUT_FILE_PATH: &str = "src/files/exercise.txt";
static OUTPUT_FILE_PATH: &str = "src/files/result.txt";

fn update_position(
    the_map: &mut Map,
    adventurer: RawAdventurer,
    new_x: isize,
    new_y: isize,
) -> RawAdventurer {
    let updated_adventurer = &adventurer.update_position(new_x, new_y);

    if updated_adventurer.position.0 >= the_map.rows
        || updated_adventurer.position.1 >= the_map.cols
    {
        return adventurer.clone();
    }

    match the_map.elements[updated_adventurer.position.0][updated_adventurer.position.1] {
        Element::RawMountain(_) => adventurer.clone(),
        Element::RawTreasure(_) => {
            Map::update_position(
                the_map,
                updated_adventurer.position.0,
                updated_adventurer.position.1,
            );
            updated_adventurer.update_treasure()
        }
        _ => updated_adventurer.clone(),
    }
}

fn get_sequence(pattern: &str) -> (char, Vec<char>) {
    let chars: Vec<char> = pattern.chars().collect();
    if let Some((first, rest)) = chars.split_first() {
        return (*first, rest.to_vec());
    }

    panic!("Input pattern is empty");
}

fn update_adventurer(adventurer: RawAdventurer, the_map: &mut Map) -> RawAdventurer {
    let pattern = adventurer.pattern.clone();
    let (action, rest_action) = get_sequence(&pattern);

    let (dx, dy, new_orientation) = match adventurer.orientation {
        Orientation::North => match action {
            'A' => (0, -1, adventurer.orientation),
            'G' => (0, 0, Orientation::West),
            'D' => (0, 0, Orientation::East),
            _ => (0, 0, adventurer.orientation),
        },
        Orientation::South => match action {
            'A' => (0, 1, adventurer.orientation),
            'G' => (0, 0, Orientation::East),
            'D' => (0, 0, Orientation::West),
            _ => (0, 0, adventurer.orientation),
        },
        Orientation::West => match action {
            'A' => (-1, 0, adventurer.orientation),
            'G' => (0, 0, Orientation::South),
            'D' => (0, 0, Orientation::North),
            _ => (0, 0, adventurer.orientation),
        },
        Orientation::East => match action {
            'A' => (1, 0, adventurer.orientation),
            'G' => (0, 0, Orientation::North),
            'D' => (0, 0, Orientation::South),
            _ => (0, 0, adventurer.orientation),
        },
    };

    RawAdventurer {
        orientation: new_orientation,
        pattern: rest_action.iter().cloned().collect::<String>(),
        ..update_position(the_map, adventurer.clone(), dx, dy)
    }
}

fn main() -> io::Result<()> {
    let file = File::open(INPUT_FILE_PATH)?;

    let map: HashMap<String, Vec<Element>> = file::file_to_hashmap(file).unwrap();

    // Get map
    let map_size: Vec<RawMap> = map
        .get("C")
        .and_then(|raw_maps| {
            Some(
                raw_maps
                    .iter()
                    .filter_map(|element| {
                        if let Element::RawMap(raw_map) = element {
                            Some(raw_map.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_else(|| panic!("No map size found"));

    // Get adventurers
    let mut raw_adventurers: Vec<RawAdventurer> = map
        .get("A")
        .and_then(|adventurers| {
            Some(
                adventurers
                    .iter()
                    .filter_map(|element| {
                        if let Element::RawAdventurer(adventurer) = element {
                            Some(adventurer.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_else(|| panic!("No adventurers found"));

    let mut the_map = map::Map::new(map_size[0].position.0, map_size[0].position.1, map.clone());

    Map::print_map(&the_map.elements);

    /* ITERATION ADVENTURER */

    loop {
        let mut all_done = true;

        for i in 0..raw_adventurers.len() {
            if !raw_adventurers[i].pattern.is_empty() {
                all_done = false;
                raw_adventurers[i] = update_adventurer(raw_adventurers[i].clone(), &mut the_map);
            }
        }

        if all_done {
            break;
        }
    }

    Map::print_map(&the_map.elements);
    let _ = generate_map_file(&the_map.to_hashmap(raw_adventurers), OUTPUT_FILE_PATH);

    Ok(())
}

#[cfg(test)]
mod tests {

    /* SHOULD ADVENTURER MOVE */

    use std::collections::HashMap;

    use parameterized::parameterized;

    use crate::{
        element::{Element, Orientation, RawAdventurer, RawMap, RawMountain, RawTreasure},
        get_sequence,
        map::Map,
        update_adventurer, update_position,
    };

    const ROW_SIZE_MAP: usize = 3;
    const COL_SIZE_MAP: usize = 4;

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

    #[test]
    fn should_adventurer_move_to_next_position() {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer =
            RawAdventurer::new("Lara".to_string(), 0, 0, Orientation::East, "A".to_string());

        // When
        let result_adventurer = update_position(&mut mock_map, fake_adventurer, 1, 0);

        // Then
        let expected_adventurer =
            RawAdventurer::new("Lara".to_string(), 1, 0, Orientation::East, "A".to_string());
        assert_eq!(expected_adventurer, result_adventurer)
    }

    #[test]
    fn should_adventurer_get_treasure_when_move_to_treasure_position() {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::South,
            "A".to_string(),
        );

        // When
        let result_adventurer = update_position(&mut mock_map, fake_adventurer, 0, 1);

        // Then
        let mut expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            1,
            Orientation::South,
            "A".to_string(),
        );
        expected_adventurer = expected_adventurer.update_treasure();

        assert_eq!(expected_adventurer, result_adventurer);
    }

    /* SHOULD ADVENTURER NOT MOVE */

    #[test]
    fn should_adventurer_do_nothing_when_moving_to_mountain_position() {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            1,
            0,
            Orientation::South,
            "A".to_string(),
        );

        // When
        let result_adventurer = update_position(&mut mock_map, fake_adventurer, 0, 1);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            1,
            0,
            Orientation::South,
            "A".to_string(),
        );

        assert_eq!(expected_adventurer, result_adventurer);
    }

    #[test]
    fn should_adventurer_not_move_when_next_position_x_is_oustide_map() {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            2,
            0,
            Orientation::South,
            "A".to_string(),
        );

        // When
        let result_adventurer = update_position(&mut mock_map, fake_adventurer, 1, 0);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            2,
            0,
            Orientation::South,
            "A".to_string(),
        );

        assert_eq!(expected_adventurer, result_adventurer);
    }

    #[test]
    fn should_adventurer_not_move_when_next_position_y_is_oustide_map() {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            3,
            Orientation::South,
            "A".to_string(),
        );

        // When
        let result_adventurer = update_position(&mut mock_map, fake_adventurer, 0, 1);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            3,
            Orientation::South,
            "A".to_string(),
        );

        assert_eq!(expected_adventurer, result_adventurer);
    }

    /* SHOULD GET SEQUENCE */

    #[test]
    fn should_get_adeventurer_action_sequence() {
        // Given
        let sequence = "DADG";

        // When
        let result = get_sequence(sequence);

        // Then
        let expected = ('D', vec!['A', 'D', 'G']);

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "Input pattern is empty")]
    fn should_panic_when_empty_action_sequence() {
        // Given
        let sequence = "";

        // When + Then
        get_sequence(sequence);
    }

    /* SHOULD UPDATE ADVENTURER */

    #[parameterized(
        orientation = { Orientation::North,  Orientation::South, Orientation::West, Orientation::East},
        coordinates = {(0, 0), (1, 2), (0, 0), (0,0)},
        expected_coordinates = {(0, 0), (1, 3), (0,0), (1,0)})]
    fn should_adventurer_not_change_orientation_when_move(
        orientation: Orientation,
        coordinates: (usize, usize),
        expected_coordinates: (usize, usize),
    ) {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            coordinates.0,
            coordinates.1,
            orientation.clone(),
            "AG".to_string(),
        );

        // When
        let adventurer_result = update_adventurer(fake_adventurer, &mut mock_map);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            expected_coordinates.0,
            expected_coordinates.1,
            orientation.clone(),
            "G".to_string(),
        );

        assert_eq!(expected_adventurer, adventurer_result);
    }

    #[parameterized(
        orientation = { Orientation::North,  Orientation::South, Orientation::West, Orientation::East },
        expected_orientation = { Orientation::West,  Orientation::East, Orientation::South, Orientation::North},
        coordinates = {(0, 0), (0, 0), (0, 0), (0, 0)})]
    fn should_adventurer_change_orientation_when_adventurer_turn_to_left(
        orientation: Orientation,
        expected_orientation: Orientation,
        coordinates: (usize, usize),
    ) {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            coordinates.0,
            coordinates.1,
            orientation,
            "GA".to_string(),
        );

        // When
        let adventurer_result = update_adventurer(fake_adventurer, &mut mock_map);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            coordinates.0,
            coordinates.1,
            expected_orientation,
            "A".to_string(),
        );

        assert_eq!(expected_adventurer, adventurer_result);
    }

    #[parameterized(
        orientation = { Orientation::North,  Orientation::South, Orientation::West, Orientation::East },
        expected_orientation = { Orientation::East,  Orientation::West, Orientation::North, Orientation::South},
        coordinates = {(0, 0), (0, 0), (0, 0), (0, 0)})]
    fn should_adventurer_change_orientation_when_adventurer_turn_to_right(
        orientation: Orientation,
        expected_orientation: Orientation,
        coordinates: (usize, usize),
    ) {
        // Given
        let mut mock_map = init_mock_map();
        let fake_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            coordinates.0,
            coordinates.1,
            orientation,
            "DA".to_string(),
        );

        // When
        let adventurer_result = update_adventurer(fake_adventurer, &mut mock_map);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            coordinates.0,
            coordinates.1,
            expected_orientation,
            "A".to_string(),
        );

        assert_eq!(expected_adventurer, adventurer_result);
    }
}
