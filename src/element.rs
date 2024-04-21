#[derive(PartialEq, Debug, Clone)]
pub enum Element {
    RawMap(RawMap),
    RawMountain(RawMountain),
    RawTreasure(RawTreasure),
    RawAdventurer(RawAdventurer),
    Empty(Empty),
}

impl Element {
    pub fn position(&self) -> Option<(usize, usize)> {
        match self {
            Element::Empty(empty) => Some(empty.position),
            Element::RawMap(map) => Some(map.position),
            Element::RawMountain(mountain) => Some(mountain.position),
            Element::RawTreasure(treasure) => Some(treasure.position),
            Element::RawAdventurer(adventurer) => Some(adventurer.position),
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn from_char(raw_value: char) -> Orientation {
        match raw_value {
            'N' => Orientation::North,
            'S' => Orientation::South,
            'E' => Orientation::East,
            'O' => Orientation::West,
            _ => panic!("Unknown orientation : {}", raw_value),
        }
    }

    pub fn from(orientation: Orientation) -> String {
        match orientation {
            Orientation::North => "N".to_string(),
            Orientation::South => "S".to_string(),
            Orientation::West => "O".to_string(),
            Orientation::East => "E".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Empty {
    position: (usize, usize),
}

impl Empty {
    pub fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RawMap {
    pub position: (usize, usize),
}

impl RawMap {
    pub fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RawMountain {
    pub position: (usize, usize),
}

impl RawMountain {
    pub fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RawTreasure {
    pub position: (usize, usize),
    pub nb_treasure: usize,
}

impl RawTreasure {
    pub fn new(x: usize, y: usize, nb_treasure: usize) -> Self {
        Self {
            position: (x, y),
            nb_treasure,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RawAdventurer {
    pub name: String,
    pub position: (usize, usize),
    pub orientation: Orientation,
    pub pattern: String,
    pub treasure: usize,
}

impl RawAdventurer {
    pub fn new(
        name: String,
        x: usize,
        y: usize,
        orientation: Orientation,
        pattern: String,
    ) -> Self {
        Self {
            name,
            position: (x, y),
            orientation,
            pattern,
            treasure: 0,
        }
    }

    pub fn update_position(&self, dx: isize, dy: isize) -> RawAdventurer {
        let (x, y) = &self.position;
        let new_x = (*x as isize + dx) as usize;
        let new_y = (*y as isize + dy) as usize;

        RawAdventurer {
            position: (new_x, new_y),
            ..self.clone()
        }
    }

    pub fn update_treasure(&self) -> RawAdventurer {
        RawAdventurer {
            treasure: self.treasure + 1,
            ..self.clone()
        }
    }

    pub fn get_orientation_to_string(&self) -> String {
        Orientation::from(self.orientation)
    }
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    /* SHOULD TEST ORIENTATION */

    #[parameterized(
        orientation = { Orientation::North,  Orientation::South, Orientation::East, Orientation::West},
        char = {'N', 'S', 'E', 'O'})]
    fn should_get_orientation_when_given_char(orientation: Orientation, char: char) {
        assert_eq!(Orientation::from_char(char), orientation);
    }

    #[test]
    #[should_panic(expected = "Unknown orientation : X")]
    fn should_panic_when_given_unknown_character() {
        let _ = Orientation::from_char('X');
    }

    /* SHOULD TEST RAWADVENTURER */

    #[test]
    fn should_init_adventurer() {
        // Given
        let adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::North,
            "AAA".to_string(),
        );

        // When + Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::North,
            "AAA".to_string(),
        );
        assert_eq!(adventurer, expected_adventurer);
    }

    #[test]
    fn should_update_adventurer_position() {
        // Given
        let adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::North,
            "AAA".to_string(),
        );

        // When
        let new_adventurer = adventurer.update_position(1, 1);

        // Then
        let expected_adventurer = RawAdventurer::new(
            "Lara".to_string(),
            1,
            1,
            Orientation::North,
            "AAA".to_string(),
        );
        assert_eq!(new_adventurer, expected_adventurer);
    }

    #[test]
    fn should_add_treasure_to_adventurer() {
        // Given
        let adventurer = RawAdventurer::new(
            "Lara".to_string(),
            0,
            0,
            Orientation::North,
            "AAA".to_string(),
        );

        // When
        let new_adventurer = adventurer.update_treasure();

        // Then
        assert_eq!(new_adventurer.position, (0, 0));
        assert_eq!(new_adventurer.orientation, Orientation::North);
        assert_eq!(new_adventurer.pattern, "AAA");
        assert_eq!(new_adventurer.treasure, 1);
    }

    #[parameterized(
        orientation = { Orientation::North,  Orientation::South, Orientation::East, Orientation::West},
        char = {'N', 'S', 'E', 'O'})]
    fn should_get_adventurer_orientation_to_string(orientation: Orientation, char: char) {
        let adventurer =
            RawAdventurer::new("Lara".to_string(), 0, 0, orientation, "AAA".to_string());

        // When + Then
        assert_eq!(adventurer.get_orientation_to_string(), char.to_string());
    }

    /* SHOULD TEST EMPTY */

    #[test]
    fn should_init_empty() {
        let empty_element = Element::Empty(Empty { position: (3, 5) });
        assert_eq!(empty_element.position(), Some((3, 5)));
    }

    /* SHOULD TEST RAWMAP */

    #[test]
    fn should_init_raw_map() {
        let raw_map_element = Element::RawMap(RawMap { position: (1, 2) });
        assert_eq!(raw_map_element.position(), Some((1, 2)));
    }

    /* SHOULD TEST RAWMOUNTAIN */

    #[test]
    fn should_init_raw_mountain() {
        let raw_mountain_element = Element::RawMountain(RawMountain { position: (1, 2) });
        assert_eq!(raw_mountain_element.position(), Some((1, 2)));
    }

    /* SHOULD INIT RAWTREASURE */

    #[test]
    fn should_init_raw_treasure() {
        let treasure = RawTreasure::new(0, 0, 10);

        assert_eq!(treasure.position, (0, 0));
        assert_eq!(treasure.nb_treasure, 10);
    }

    /* SHOULD TEST ELEMENT */

    #[parameterized(
        element = {
            Element::Empty(Empty::new(3, 5)),
            Element::RawMap(RawMap::new(1, 2)),
            Element::RawMountain(RawMountain::new(4, 6)),
            Element::RawTreasure(RawTreasure::new(2, 3, 0)),
            Element::RawAdventurer(RawAdventurer::new("LARA".to_string(), 0, 0, Orientation::North, "A".to_string())),
        },
        expected_position = {
            Some((3, 5)),
            Some((1, 2)),
            Some((4, 6)),
            Some((2, 3)),
            Some((0, 0)),
        }
    )]
    fn should_get_position_for_an_element(
        element: Element,
        expected_position: Option<(usize, usize)>,
    ) {
        let actual_position = element.position();
        assert_eq!(actual_position, expected_position);
    }
}
