#![allow(non_upper_case_globals)]

#[derive(Default, Copy, Clone)]
pub struct StageInfo {
    pub name:        &'static str,
    pub pretty_name: &'static str,
    pub layers:      &'static [u8],
    pub rooms:       &'static [u8],
    // pub entrances: &'static [u8],
}

pub struct StageCategory {
    pub name:   &'static str,
    pub stages: &'static [StageInfo],
}

macro_rules! define_stages {
    ($(($name:expr, $pretty_name:expr, $layers:expr, $rooms:expr)),*)  => (
        [$(
            StageInfo {
                name: $name,
                pretty_name: $pretty_name,
                layers: &$layers,
                rooms: &$rooms,
            },
        )*]
    )
}

pub const THE_SKY: StageCategory = StageCategory {
    name:   "The Sky",
    stages: &define_stages!(
        (
            "F000",
            "F000 - Skyloft",
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 20, 28],
            [0]
        ),
        (
            "F001r",
            "F001r - Knight Academy",
            [0, 1, 2, 3, 4, 13, 14],
            [0, 1, 2, 3, 4, 5, 6]
        ),
        ("F002r", "F002r - Beedle's Shop", [0, 1, 2], [0]),
        ("F004r", "F004r - Bazaar", [0, 1, 3], [0]),
        (
            "F005r",
            "F005r - Orielle and Parrow's House",
            [0, 1, 2],
            [0]
        ),
        ("F006r", "F006r - Kukiel's House", [0, 1, 2], [0]),
        ("F007r", "F007r - Piper's House", [0, 1, 2], [0]),
        ("F008r", "F008r - Goddess Statue", [0, 1], [0]),
        ("F009r", "F009r - Sparring Hall", [0], [0]),
        ("F010r", "F010r - Isle of Songs", [0, 1], [0]),
        ("F011r", "F011r - Lumpy Pumpkin", [0, 1, 2, 12], [0]),
        ("F012r", "F012r - Batreaux's House", [0, 1, 2, 4], [0]),
        ("F013r", "F013r - Sparrot's House", [0, 1], [0]),
        ("F014r", "F014r - Bertie's House", [0, 2], [0]),
        ("F015r", "F015r - Gondo's House", [0, 1, 2], [0]),
        ("F016r", "F016r - Pipit's House", [0, 2], [0]),
        ("F017r", "F017r - Rupin's House", [0, 2], [0]),
        ("F018r", "F018r - Peatrice's House", [0, 1, 2], [0]),
        ("F019r", "F019r - Bamboo Island", [0, 2], [0]),
        ("F020", "F020 - The Sky", [0, 1, 2, 3, 4, 6], [0]),
        ("F021", "F021 - Cutscene Sky", [0], [1]),
        ("F023", "F023 - Thunderhead", [0, 1, 2, 13], [0])
    ),
};

pub const FARON: StageCategory = StageCategory {
    name:   "Faron",
    stages: &define_stages!(
        ("F100", "F100 - Faron Woods", [0, 1, 2, 3, 4, 5], [0]),
        ("F100_1", "F100_1 - Great Tree", [0, 1, 2, 3], [0]),
        ("F101", "F101 - Deep Woods", [0, 1, 2, 3, 4, 5], [0]),
        ("F102", "F102 - Lake Floria", [0, 1, 2, 3], [0, 1, 2, 3, 4]),
        (
            "F102_1",
            "F102_1 - Outside Ancient Cistern",
            [0, 1, 2, 3],
            [0]
        ),
        ("F102_2", "F102_2 - Dragon's Lair", [0, 1, 3, 4], [0]),
        ("F103", "F103 - Flooded Faron Woods", [0, 1, 2], [0]),
        ("F103_1", "F103_1 - Flooded Great Tree", [0, 1, 2], [0])
    ),
};
pub const ELDIN: StageCategory = StageCategory {
    name:   "Eldin",
    stages: &define_stages!(
        (
            "F200",
            "F200 - Eldin Volcano",
            [0, 1, 2, 3, 4],
            [0, 1, 2, 3, 4, 5, 6, 7, 8]
        ),
        ("F201_1", "F201_1 - Volcano Summit", [0, 2, 3, 4], [0]),
        ("F201_2", "F201_2 - Bokoblin Base Summit", [0], [0]),
        ("F201_3", "F201_3 - Outside Fire Sanctuary", [0, 2, 3], [0]),
        ("F201_4", "F201_4 - Summit Waterfall", [0, 3], [0]),
        (
            "F202",
            "F202 - Bokoblin Base",
            [0, 1],
            [0, 1, 2, 3, 4, 5, 6]
        ),
        ("F202_1", "F202_1 - Lower Boko Base Cave", [0, 1], [0]),
        ("F202_2", "F202_2 - Upper Boko Base Cave", [0], [0]),
        ("F202_3", "F202_3 - Lower Eldin Cave", [0], [0]),
        ("F202_4", "F202_4 - Upper Eldin Cave", [0], [0]),
        ("F210", "F210 - Mogma Turf", [0, 1, 2, 3, 4], [0]),
        ("F211", "F211 - Thrill Digger Cave", [0, 1, 2, 3, 4], [0]),
        ("F221", "F221 - Fire Dragon's Lair", [0, 2], [0])
    ),
};

pub const LANAYRU: StageCategory = StageCategory {
    name:   "Lanayru",
    stages: &define_stages!(
        ("F300", "F300 - Lanayru Desert", [0, 1, 2], [0]),
        (
            "F300_1",
            "F300_1 - Lanayru Mine",
            [0, 1, 2, 3, 4],
            [0, 1, 2]
        ),
        ("F300_2", "F300_2 - Lightning Node", [0], [0]),
        ("F300_3", "F300_3 - Fire Node", [0], [0]),
        ("F300_4", "F300_4 - Temple of Time", [0, 1, 2, 13], [0]),
        ("F300_5", "F300_5 - LMF to ToT", [0], [0]),
        ("F301", "F301 - Ancient Harbor", [0], [0]),
        (
            "F301_1",
            "F301_1 - Lanayru Sand Sea",
            [0, 1, 2, 3, 4, 5],
            [0]
        ),
        (
            "F301_2",
            "F301_2 - Inside Pirate Stronghold",
            [0, 3, 5],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        ),
        ("F301_3", "F301_3 - Skipper's Retreat", [0, 1, 2, 3], [0]),
        ("F301_4", "F301_4 - Shipyard", [0, 1, 2, 3, 4, 5], [0]),
        ("F301_5", "F301_5 - Skipper's Shack", [0], [0]),
        ("F301_6", "F301_6 - Pirate Stronghold", [0, 1, 2, 5], [0]),
        (
            "F301_7",
            "F301_7 - Shipyard Construction Bay",
            [0, 1, 2],
            [0]
        ),
        (
            "F302",
            "F302 - Lanayru Gorge",
            [0, 1, 2, 13],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        ),
        ("F303", "F303 - Lanayru Caves", [0, 1, 2, 3, 4], [0])
    ),
};
pub const SEALED_GROUNDS: StageCategory = StageCategory {
    name:   "Sealed Grounds",
    stages: &define_stages!(
        (
            "F400",
            "F400 - Behind the Temple",
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 13],
            [0, 1]
        ),
        (
            "F401",
            "F401 - Sealed Grounds",
            [0, 1, 2, 3, 4, 5, 6, 7],
            [1]
        ),
        (
            "F402",
            "F402 - Sealed Temple",
            [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 16, 17, 18, 19],
            [0, 2]
        ),
        (
            "F403",
            "F403 - Hylia's Realm",
            [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15],
            [1]
        ),
        (
            "F404",
            "F404 - Temple of Hylia",
            [0, 1, 2, 3, 13, 14, 15],
            [0, 2]
        ),
        ("F405", "F405 - Sealed Grounds Cutscene", [0], [0]),
        (
            "F406",
            "F406 - Post-Sky-Keep Sealed Grounds",
            [0, 1, 2, 13],
            [1]
        ),
        ("F407", "F407 - Sealed Temple Cutscene", [0, 13], [0])
    ),
};
pub const DUNGEONS: StageCategory = StageCategory {
    name:   "Dungeons",
    stages: &define_stages!(
        ("D000", "D000 - Waterfall Cave", [0, 1], [0]),
        (
            "D100",
            "D100 - Skyview Temple",
            [0, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        ),
        (
            "D101",
            "D101 - Ancient Cistern",
            [0, 1, 2, 4, 5, 6],
            [0, 1, 2, 3, 4, 5, 7, 10]
        ),
        ("D200", "D200 - Earth Temple", [0], [0, 1, 2, 3, 4]),
        (
            "D201",
            "D201 - Fire Sanctuary Outer Map",
            [0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        ),
        (
            "D201_1",
            "D201_1 - Fire Sanctuary Inner Map",
            [0, 1],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        ),
        (
            "D300",
            "D300 - Lanayru Mining Facility First Rooms",
            [0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        ),
        (
            "D300_1",
            "D300_1 - Lanayru Mining Facility Hub Room",
            [0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        ),
        (
            "D301",
            "D301 - Sandship",
            [0, 1, 2, 3, 4, 9, 10, 11, 12],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        ),
        (
            "D301_1",
            "D301_1 - Sandship (Tentalus Escape)",
            [0, 2, 3],
            [0]
        ),
        ("B100", "B100 - Ghirahim 1", [0, 1, 2, 3, 4, 5, 13], [0]),
        (
            "B100_1",
            "B100_1 - Skyview Spring",
            [0, 1, 2, 3, 4, 13],
            [0]
        ),
        ("B101", "B101 - Koloktos", [0, 1, 2, 3, 6], [0]),
        ("B101_1", "B101_1 - Farore's Flame Room", [0, 1], [0]),
        ("B200", "B200 - Scaldera", [0, 1, 2, 3], [4, 10]),
        ("B201", "B201 - Ghirahim 2", [0, 1, 2, 3, 13], [0]),
        ("B201_1", "B201_1 - Din's Flame Room", [0], [0]),
        ("B300", "B300 - Moldarach", [0, 1, 2, 3], [0]),
        ("B301", "B301 - Tentalus", [0, 1, 2, 3, 13], [0]),
        ("B400", "B400 - Demise", [0, 1], [0])
    ),
};
pub const SILENT_REALMS: StageCategory = StageCategory {
    name:   "Silent Realms",
    stages: &define_stages!(
        ("S000", "S000 - Skyloft Silent Realm", [0, 2], [0]),
        ("S100", "S100 - Faron Silent Realm", [0, 2], [0]),
        ("S200", "S200 - Eldin Silent Realm", [0, 2], [0]),
        ("S300", "S300 - Lanayru Silent Realm", [0, 2], [0])
    ),
};
