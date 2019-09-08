use arrayvec::ArrayString;
use core::fmt::Write;
use libtp::system::boss_flags_value;
use libtp::link::Inventory;
use utils::*;
use {controller, get_state, visible, warping};
use print;

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    const INVENTORY_INDEX: usize = 0;
    const CHEAT_INDEX: usize = 1;
    const WARPING_INDEX: usize = 2;
    const MEMORY_INDEX: usize = 3;
    const SETTINGS_INDEX: usize = 4;
    const QUICK_WARP_INDEX: usize = 6;
    const ALTER_BOSS_FLAGS_INDEX: usize = 8;
    const ALTER_RUPEE_TEXT_INDEX: usize = 9;

    let state = unsafe { get_state() };
    let inventory = Inventory::get_inventory();
    let boss_flags = boss_flags_value();
    let lines = state.menu.lines_mut();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            visible = false;
        }
        return;
    }

    let contents = [
        "Inventory",
        "Cheats",
        "Warping",
        "Memory",
        "Settings",
        "",
        "Quick Warp",
        "",
        "Set/Clear Boss Flags",
        "Set/Clear Rupee Text",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        match unsafe { cursor } {
            MEMORY_INDEX => {
                transition(MenuState::Memory);
                return;
            }
            INVENTORY_INDEX => {
                transition(MenuState::InventoryMenu);
                return;
            }
            CHEAT_INDEX => {
                transition(MenuState::CheatMenu);
                return;
            }
            SETTINGS_INDEX => {
                transition(MenuState::Settings);
                return;
            }
            WARPING_INDEX => {
                transition(MenuState::Warp);
                return;
            }
            QUICK_WARP_INDEX => {
                warping::load_saved_warp();
                return;
            }
            ALTER_BOSS_FLAGS_INDEX => {
                if *boss_flags == 0 {
                    *boss_flags = 10;
                } else {
                    *boss_flags = 0;
                }
            }
            ALTER_RUPEE_TEXT_INDEX => {
                if inventory.rupee_cs_flags == 0 {
                    inventory.rupee_cs_flags = 0xFF;
                } else {
                    inventory.rupee_cs_flags = 0;
                }
            }
            _ => {}
        }
    }

    for (index, (line, &content)) in lines
        .iter_mut()
        .zip(&contents)
        .enumerate() {
            let _ = match index {
                ALTER_BOSS_FLAGS_INDEX => write!(line.begin(), "{}: {}", content, boss_flags),
                ALTER_RUPEE_TEXT_INDEX => write!(line.begin(), "{}: {:02X}", content, inventory.rupee_cs_flags),
                _ => write!(line.begin(), "{}", content),
            };
            line.selected = index == unsafe { cursor };
        }
    }

pub fn render_descriptions() {
        let mut descriptions = [
            "Modify Link's item wheel and pause menu items",
            "Apply some cheats!",
            "Warp to a location",
            "Manage memory address watches",
            "Save/load your settings",
            "",
            ""
        ];
        let mut s = ArrayString::<[u8; 64]>::new();

        unsafe {
            // prevent oob index
            if cursor < descriptions.len() {
                let _ = write!(s, "{}", descriptions[cursor]);
                print::printf(s.as_str(), 20.0, 425.0, 0x77_88_99_FF);
            }
        }
}