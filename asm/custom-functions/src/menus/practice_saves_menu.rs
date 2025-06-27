use crate::system::button::*;
use crate::utils::menu::SimpleMenu;
use crate::utils::practice_saves::load_practice_save;
use alloc::vec;
use alloc::vec::Vec;

use super::main_menu;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PracticeSavesMenuState {
    Off,
    Main,
    Category,
}

pub struct PracticeSavesMenu {
    state:       PracticeSavesMenuState,
    cursor:      u32,
    save_cursor: u32,
    categories:  Vec<SpeedrunCategory>,
}

#[no_mangle]
#[link_section = "data"]
static mut PRACTICE_SAVES_MENU: PracticeSavesMenu = PracticeSavesMenu {
    state:       PracticeSavesMenuState::Off,
    cursor:      0,
    save_cursor: 0,
    categories: Vec::new(),
};

struct SpeedrunCategory {
    name:      &'static str,
    base_path: &'static str,
    saves:     Vec<(&'static str, &'static str)>, // path, description
    description: &'static str,
}

impl SpeedrunCategory {
    fn num_saves(&self) -> u32 {
        self.saves.len() as u32
    }
}

impl super::Menu for PracticeSavesMenu {
    fn enable() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state = PracticeSavesMenuState::Main;
    }

    fn disable() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state = PracticeSavesMenuState::Off;
    }
    fn input() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };

        const SAVE_FILE: u32 = 0;
        const LOAD_FILE: u32 = 1;
        const LOAD_FILE_DIRECT: u32 = 2;
        const GIVE_ITEM: u32 = 3;
        const KILL_LINK: u32 = 4;
        const PRAC_SAVE: u32 = 5;

        match ps_menu.state {
            PracticeSavesMenuState::Off => {},
            PracticeSavesMenuState::Main => {
                if is_pressed(B) {
                    ps_menu.state = PracticeSavesMenuState::Off;
                } else if is_pressed(A) {
                    ps_menu.state = PracticeSavesMenuState::Category;
                    let category = &ps_menu.categories[ps_menu.cursor as usize];
                    if ps_menu.save_cursor >= category.num_saves() {
                        ps_menu.save_cursor = 0;
                    }
                }
            },
            PracticeSavesMenuState::Category => {
                let category = &ps_menu.categories[ps_menu.cursor as usize];
                if is_pressed(B) {
                    ps_menu.state = PracticeSavesMenuState::Main;
                } else if is_pressed(A) {
                    let save = category.saves[ps_menu.save_cursor as usize].0;
                    load_practice_save(format!("{0}/{save}", category.base_path).as_str());
                    ps_menu.state = PracticeSavesMenuState::Off;
                    main_menu::MainMenu::disable();
                }
            },
        }
    }
    fn display() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };

        match ps_menu.state {
            PracticeSavesMenuState::Off => {},
            PracticeSavesMenuState::Main => {
                let menu = crate::reset_menu();
                menu.set_heading("Choose a Category");
                for category in &ps_menu.categories {
                    menu.add_entry_fmt(format_args!("{}", category.name), category.description);
                }

                menu.set_cursor(ps_menu.cursor);
                menu.draw();
                ps_menu.cursor = menu.move_cursor();
            },
            PracticeSavesMenuState::Category => {
                let category = &ps_menu.categories[ps_menu.cursor as usize];
                let menu = crate::reset_menu();
                menu.set_heading("Choose a Practice Save");
                for (idx, save) in category.saves.iter().enumerate() {
                    menu.add_entry_fmt(format_args!("{}: {}", idx, save.0), save.1);
                }
                menu.set_cursor(ps_menu.save_cursor);
                menu.draw();
                ps_menu.save_cursor = menu.move_cursor();
            },
        }
    }

    fn is_active() -> bool {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state != PracticeSavesMenuState::Off
    }
}

pub fn initialize_practice_saves() {
    let any_percent = SpeedrunCategory {
        name:      "Any%",
        base_path: "/saves/Any",
        saves:     vec![
            ("Start", "2 blank Hero Mode files"),
            ("First BiT", "Save before the first instance of Back in Time"),
            ("Copy After Cave", "Save after the copy after Waterfall Cave"),
            ("Sky RBW", "Hacked save with F1 ready for BiT into Sky RBW"),
            ("Skyview RBW", "Save just after entering Faron"),
            ("F3 in Skyview", "Save with File 3 in Skyview Temple"),
            ("Ghirahim 1", "Save with File 3 just before Ghirahim 1"),
            ("Goddess Statue RBW", "Save just after completing Skyview Temple"),
            ("Eldin RBW", "Save at the tunic prompt, before the RBW into Eldin"),
            ("Eldin OoB", "Hacked save with File 3 in OoB Eldin Volcano"),
            ("ET Door RBM", "Save just before the RBM to open ET"),
            ("ET Bridge RBM", "Save just before the RBM to raise the ET main bridge"),
            ("F1 Keese Yeet F2 Scaldera", "Save with F1 at the start of ET, F2 just before Scaldera"),
            ("Lanayru Pillar RBM", "Save just after completing Earth Temple"),
            ("Lanayru Mine BiTWarp", "Save just after entering Lanayru"),
            ("Rock RBM", "Save just before the RBM to blow up the Lanayru Gorge rock"),
            ("Machi RBM", "Save just before the RBM to activate Minecart Escort"),
            ("Gorge BiTWarp", "Save just before the BiTWarp in Lanayru Gorge"),
            ("2x20 Crystal RBM", "Save just before the final RBM in Lanayru Gorge"),
            ("3 in 1 - G3 Escape, Statue, Demise", "Save with F1 at Boss Rush, F2 at the OoB Hylia's Realm statue, F3 before Demise"),
        ],
        description: "Saves for the Ghirahim 3 Escape Fast Faron BiT Any% route."
    };
    let all_dungeons = SpeedrunCategory {
        name:      "All Dungeons",
        base_path: "/saves/All Dungeons",
        saves:     vec![
            ("Start", "2 blank Hero Mode files"),
            ("After Waterfall Cave", "Save after the copy after Waterfall Cave"),
            ("Sealed Grounds", "Save just after entering Faron"),
            ("Behind the Temple", "BiTSaved at the Behind the Temple statue"),
            ("Deep Woods", "Hacked save at the start of Deep Woods before Skyview Temple"),
            ("Skyview", "Saved at the start of Skyview Temple"),
            ("After Skyview", "Saved just after completing Skyview Temple"),
            ("Volcano Ascent", "Save at the Volcano Ascent statue in Eldin 1"),
            ("Earth Temple", "Save at the start of Earth Temple"),
            ("Scaldera", "Hacked save in the Scaldera boss arena"),
            ("After ET", "Save just after completing Earth Temple"),
            ("AC CSWW", "Save just before the Cutscene Skip Wrong Warp into Ancient Cistern"),
            ("Ancient Cistern", "Save at the start of Ancient Cistern"),
            ("After Cistern", "Save just after completing Ancient Cistern"),
            ("Stone Cache", "Save at the Stone Cache statue in Lanayru 1"),
            ("Raise LMF", "Save just before the RBM to raise the Lanayru Mining Facility"),
            ("Sand Sea Skip", "Save just before the RBM to enter Sandship early"),
            ("Sandship", "Save at the first statue in Sandship"),
            ("Lanayru Mining Facility", "Save at the start of Lanayru Mining Facility"),
            ("After LMF", "Save just after completing Lanayru Mining Facility"),
            ("Sky Keep", "Save at the start of Sky Keep"),
            ("After Sky Keep", "Save just after completing Sky Keep"),
            ("Eldin Trial RBM", "Save just before the RBM to open the Eldin Silent Realm"),
            ("After Eldin Trial", "Save just after completing the Eldin Silent Realm"),
            ("Fire Sanctuary", "Save at the start of Fire Sanctuary"),
            ("Gate of Time Skip", "Save at the prompt after FS, before the CSWW to skip the Gate of Time"),
            ("Horde", "Save in Temple of Hylia before the final boss gauntlet"),
        ],
        description: "Saves for the CSWW No EBR Fast Faron BiT All Dungeons route."
    };
    let hundo = SpeedrunCategory {
        name:      "100% (v5.1.3 Route)",
        base_path: "/saves/100 v5_1_3",
        saves: vec![
            ("Start", "2 blank Hero Mode files with 99 of every treasure and bug."),
            ("Copy After Cave", "Save after the copy after Waterfall Cave"),
            ("Sealed Grounds", "Save just after entering Faron"),
            ("Fi Escort", "Save after getting Sailcloth, before Sealed Grounds Skip"),
            ("Faron Entry Statue", "Save after performing Sealed Grounds Skip"),
            ("Skyview Start", "Save at the start of Skyview Temple"),
            ("Skyview 1 After Copy", "Save in Skyview Temple after obtaining the Beetle and copying F1 -> F2"),
            ("Skyview Prompt", "Save at the prompt after Skyview Temple"),
            ("Volcano Entry", "Save at the Eldin Volcano Entry statue"),
            ("ET Door RBM", "Save just before the RBM to open ET"),
            ("ET Start", "Save at the start of Earth Temple"),
            ("Scaldera", "Save just before fighting Scaldera in Earth Temple"),
            ("First Batreaux RBM", "Save after ET before the first Batreaux RBM"),
            ("ToT Statue RBM", "Save after entering Lanayru, before RBM for early ToT statue"),
            ("Gorge RBMs", "Save at Lanayru Gorge before various RBMs"),
            ("Early Boko Base RBW", "Save before RBW into Bokoblin Base for early items"),
            ("Faron Trial RBW", "Save before RBW into Faron Silent Realm"),
            ("Cistern RBW", "Save before RBW into Ancient Cistern"),
            ("Inside Cistern", "Save at the statue near the spider's thread in Ancient Cistern"),
            ("After Impa", "Save after obtaining beacons from Impa in Sealed Temple"),
            ("Raise LMF RBM", "Save before RBM to open Lanayru Mining Facility"),
            ("LMF Start", "Save at the start of Lanayru Mining Facility"),
            ("Sharkhead RBM", "Save before RBM to open up the Pirate Stronghold Sharkhead"),
            ("Skyloft 3", "Save at the start of the third major Skyloft segment (after getting pumpkin soup)"),
            ("Ballad RBM", "Save before RBM to obtain Ballad of the Goddess"),
            ("ELTS", "Save before RBM to obtain Life Tree Seedling early"),
            ("After Shipyard", "Save after completing the Shipyard in Sand Sea"),
            ("After Skippers", "Save after completing Skipper's Retreat in Sand Sea"),
            ("Eldin RBW", "Save before RBW to the start of Eldin Volcano"),
            ("Gate of Time RBM", "Save before RBM to open the Gate of Time early"),
            ("Skyloft 4", "Save at the start of the fourth major Skyloft segment (after Gorko's heart piece)"),
            ("Levias", "Save before fighting Levias & Bilocyte at the Thunderhead"),
            ("Boko Base RBW", "Save before second RBW to complete Bokoblin Base"),
            ("FS Flame Wall RBM", "Save before RBM to remove flames in front of Fire Sanctuary"),
            ("FS Start", "Save at the start of Fire Sanctuary"),
            ("Skyloft 5", "Save at the start of the fifth major Skyloft segment (after FS)"),
            ("Volcano East", "Save at Volcano East for Eldin cleanup before SotH segments"),
            ("Imprisoned 3", "Save before fighting the third version of The Imprisoned"),
            ("After Imp 3", "Save after defeating Imprisoned 3, before Tadtones"),
            ("After Tadtones", "Save after obtaining Faron's part of the Song of the Hero"),
            ("Boss Rush", "Save before playing the Boss Rush minigame"),
            ("Farores Courage RBM", "Save before RBM to obtain Farore's Courage"),
            ("Skyloft 6", "Save at the start of the sixth major Skyloft segment (after Thunderhead cleanup)"),
            ("Sky Keep Start", "Save at the start of Sky Keep"),
            ("Courage Lever RBM", "Save before RBM to open the bars to the Triforce of Courage early"),
            ("Horde", "Save in Temple of Hylia before the final boss gauntlet"),
        ],
        description: "Saves for the v5.1.3 Imp1 Skip + Fast Faron BiT 100% route."
    };
    let hundo_sc_delay = SpeedrunCategory {
        name:      "100% (v5.2.0 Route)",
        base_path: "/saves/100 v5_2_0",
        saves: vec![
            ("Start", "2 blank Hero Mode files with 99 of every treasure and bug."),
            ("Copy After Cave", "Save after the copy after Waterfall Cave"),
            ("Sealed Grounds", "Save just after entering Faron the first time"),
            ("Faron Entry Statue", "Save after performing Sealed Grounds Skip"),
            ("Deep Woods", "Save in Deep Woods before Skyview"),
            ("Skyview 1 Start", "Save at the start of Skyview Temple"),
            ("Skyview 1 After Copy", "Save in Skyview Temple after obtaining the Beetle and copying F1 -> F2"),
            ("Skyview 1 Ghirahim", "Save in Skyview Temple before fighting Ghirahim"),
            ("After Skyview", "Save at the prompt after Skyview Temple"),
            ("Before Sealed Grounds 1", "Hacked save at the start of the return to Sealed Grounds"),
            ("Eldin RBW", "Save before the Reverse BiTWarp to Eldin"),
            ("ET Door RBM", "Save just before the RBM to open ET"),
            ("ET Start", "Save at the start of Earth Temple"),
            // ("Scaldera", "Save just before fighting Scaldera in Earth Temple"),
            ("Batreaux Inside Door RBM", "Save after ET before the Batreaux door RBM"),
            ("Lanayru Mine Entry", "Save after entering Lanayru the first time"),
            ("Lanayru Gorge 2x20 RBM", "Save at Lanayru Gorge before the Thunder Dragon healing RBM"),
            ("Machi RBM", "Save at Lanayru Gorge before the escort skip RBM"),
            ("Gorge BiTWarp", "Save before the OoB BiTWarp in Lanayru Gorge"),
            ("Early Boko Base RBW", "Save before RBW into Bokoblin Base for early items"),
            ("Faron Trial RBW", "Save before RBW into Faron Silent Realm"),
            ("Cistern RBW", "Save before RBW into Ancient Cistern"),
            ("Lake Floria", "Save after Ancient Cistern at the Floria Waterfall statue"),
            ("Raise LMF RBM", "Save before RBM to open Lanayru Mining Facility"),
            ("LMF Start", "Save at the start of Lanayru Mining Facility"),
            ("Moldarach", "Save just before fighting Moldarach in LMF"),
            ("After LMF", "Save just after completing Lanayru Mining Facility"),
            ("Sharkhead RBM", "Save before RBM to open up the Pirate Stronghold Sharkhead"),
            ("Skyloft 3", "Save at the start of the third major Skyloft segment (after getting pumpkin soup)"),
            ("Ballad RBM", "Save before RBM to obtain Ballad of the Goddess"),
            ("Early Thunderhead", "Save before RBM to open the Thunderhead"),
            ("ELTS", "Save before RBM to obtain Life Tree Seedling early"),
            ("After Shipyard", "Save after completing the Shipyard in Sand Sea"),
            ("Sandship Start", "Save at the start of Sandship"),
            ("Eldin 2 RBW", "Save before the second RBW to the start of Eldin Volcano"),
            ("Gate of Time RBM", "Save before RBM to open the Gate of Time early"),
            ("Sailcloth Setup RBW", "Save before the RBW to setup obtaining the Sailcloth"),
            ("Party Wheel Pickup", "Save in Lanayru before obtaining the Party Wheel"),
            ("Northeast Sky Cleanup", "Save in the Sky before opening various Goddess Chests"),
            ("Boko Base RBW", "Save before second RBW to complete Bokoblin Base"),
            ("FS Flame Wall RBM", "Save before RBM to remove flames in front of Fire Sanctuary"),
            ("FS Start", "Save at the start of Fire Sanctuary"),
            ("Skyloft 5", "Save at the start of the fifth major Skyloft segment (after FS)"),
            ("Lumpy Pumpkin", "Save before completing Lumpy Pumpkin sidequests"),
            ("Volcano East", "Save at Volcano East for Eldin cleanup before SotH segments"),
            ("Imprisoned 3", "Save before fighting the third version of The Imprisoned"),
            ("Tadtones", "Save in the Sealed Temple before the Tadtones segment"),
            ("After Tadtones", "Save after obtaining Faron's part of the Song of the Hero"),
            ("Boss Rush", "Save before playing the Boss Rush minigame"),
            ("Farores Courage RBM", "Save before RBM to obtain Farore's Courage"),
            ("Skyloft 6", "Save at the start of the sixth major Skyloft segment (after Thunderhead cleanup)"),
            ("After Skyloft Trial", "Save after completing the Skyloft Silent Realm"),
            ("Courage Lever RBM", "Save before RBM to open the bars to the Triforce of Courage early"),
            ("Horde", "Save in Temple of Hylia before the final boss gauntlet"),
        ],
        description: "Saves for the v5.2.0 Sailcloth Delay 100% route."
    };
    unsafe {
        PRACTICE_SAVES_MENU.categories.push(any_percent);
        PRACTICE_SAVES_MENU.categories.push(all_dungeons);
        PRACTICE_SAVES_MENU.categories.push(hundo);
        PRACTICE_SAVES_MENU.categories.push(hundo_sc_delay);
    }
}