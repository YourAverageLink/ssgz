# ssgz 0.5.0 (name pending)

A ROM hack for _The Legend of Zelda: Skyward Sword_ aimed to help out speedrunners with practice and research.

## Downloads / Usage

ssgz is a program that may be run from source or run as an executable from the Releases tab. It features a UI from which you may extract a copy of Skyward Sword and apply practice patches.

It can optionally accept the `--noui` argument to run solely in the command line, along with another argument for the version you wish to patch: `us` for the NTSC North American 1.0 version, or `jp` for the Japanese version.

You will need to provide a clean `.iso` file of either of these versions (version code `SOUE01` or `SOUJ01`, respectively).

### Running from Source

- Install Rust (if necessary): https://rustup.rs/
- Clone this repo

  `git clone https://github.com/YourAverageLink/ssgz`

  `cd ssgz`

- Run (release mode is recommended for optimal performance): `cargo run --release`

ssgz uses Dioxus to power its UI, so you may also wish to install the [Dioxus CLI for hotreloading with `dx serve --release`](https://dioxuslabs.com/learn/0.6/getting_started/#)

### Running an executable:

- Windows / Linux:
  - Simply open the executable (or optionally, run with no UI in the command line with ) `ssgz.exe --noui [us | jp]` on Windows or `./ssgz --noui [us | jp]`
  - Note that the included `assets` directory must be in the same folder as the executable for the css to properly load
- MacOS
  - Unfortunately, the MacOS `.app` file won't work on a fresh install due to code signing issues. However, you can make it executable after download by double-clicking the included `RUN_BEFORE_USE.command` file.
  - If you wish to run with noui, you will need to run `./Ssgz.app/Contents/MacOS/ssgz --noui [us | jp]`
  - The `.app` file includes its own assets, so you won't need an assets folder

When running with no UI, the program will automatically perform the extract and patching steps below, asking if you want to create a patched ISO.

To setup patches for your version of the game, click `Extract ISO`. It will then ask for your copy of _Skyward Sword_. Once you select a valid iso, it will begin extracting it to `extract/[version]`, while copying the original `main.dol` file to `original-dol/[version]`. A progress bar will display the extraction progress. **Though some large files (namely hint videos / credits videos) are removed to save on space,
note that the extract will still take up more than 2 GB of space.**

You may now click `Write Patched ISO`, and it will then copy a patched dol file over `main.dol` in the extract, copy practice saves, and copy the custom REL file.

Once patching is done, it will ask where to put the a patched ISO. A progress bar will display the ISO packing progress. You may play this ISO through Dolphin or on console with a USB loader.

## Features / In-Game Usage

While in gz, pressing Z and C simultaneously on the Nunchuck will open up a Practice Menu, with some submenus. A description of each menu item is visible on the bottom of the screen, along with basic control info.

### Display Menu

In the Display Menu, you may toggle whether or not certain information should be passively displayed on-screen

- **Input Viewer** will show a display of Wii Remote and Nunchuck inputs on the bottom of the screen.
- **Link Pos Viewer** will show Link's x, y, and z coordinates, facing angle, and current movement speed.
- **Scene Flag Viewer** will display the scene flags and temporary flags active in the current scene.
- **Frame Count Viewer** will display the number of in-game frames that have passed since the last load, as well as a conversion to `mm:ss.xx`
- **FreeCam** will allow for freeform camera movement toggled by pressing 2 and Z. Controls will be displayed in the menu. Note that, while FreeCam is active, you may not open the menu again, so to freeze the camera in place, use this in conjunction with Freeze Camera.
- **Freeze Camera** will lock the camera in its current position.
- **Enemy Info Viewer** will display the coordinates of the last actor you've targeted.

### Warp Menu

In the Warp Menu, you may choose a stage you wish to warp to, along with which room, layer, and entrance you wish to use.
Note that some entrances / layers may crash, so use this at your own risk.

### Action Menu

The Action Menu contains some useful miscellaneous functions.

- **Save File** acts like pressing D-Pad Right in the old practice Gecko codes -- it saves your stage, position, flags, etc., which may be loaded by the two following options
- **Load File** will reload the area you saved in with **Save File**, at the last entrance you took on that file.
- **Direct Load File** will reload the file you saved with **Save File** with the coordinates you saved at as well.
- **Load Position** will set Link's position to that of the file you saved with **Save File**, without reloading the map.
- **Kill Link** kills Link (even if you have the **Infinite Health** cheat enabled).
- **RBM Scene Flag** pulls up a submenu where you may select a scene flag to RBM (& commit) in the current area.
- **Enter BiT** will load into Back in Time on Skyloft.
- **Control RNG** will allow you to toggle fixed random number generation; you may select floating point numbers from 0 to 1 (in increments of 0.01) for the RNG function to always return.
- **Simulate EB** will initiate a fatal blow targeting the origin of the map if you're targeting anything.

### Cheats Menu

The Cheats Menu contains some useful cheats that will remain active as long as they are toggled here. The cheats include...

- Infinite Health
- Infinite Stamina
- Infinite Slingshot Seeds
- Infinite Bombs
- Infinite Arrows
- Infinite Rupees
- Moon Jump (only active while holding D-Pad Right)
- Instant Text (only as fast as the old Randomizer instant text as current rando instant text changes text RBM timing)
- Hide UI

### Practice Saves Menu

The Practice Saves Menu allows you to load practice saves for certain speedrun categories, useful for practicing certain parts of the run. Saves are included for...

- Any% (on the Ghirahim 3 Escape + Fast Faron BiT route, the recommended route at the time of writing)
- All Dungeons (on the Fast Faron BiT CSWW route, the recommended route at the time of writing)
- 100% (sets for both the Imp1 Skip + Fast Faron BiT route and the Sailcloth delay route)
- Any% Restricted BiT (on the Ghirahim 3 Escape route with the bed/death trick to skip Scrapper Escort)
- Any% No BiT (where files 1 through 3 are described in the description field for each save)

### Flag Menu

The Flag Menu contains a list of some important story flags for game progress.
Each entry will display `[x]` if it is currently set, and `[]` if it is not. You may press A on each flag to toggle it.

### Inventory Menu

The Inventory Menu contains a list of notable items. Each entry displays the current level of the item you have (for instance, "Goddess Sword" on the Sword row if that's what you have).
For each item, you may use D-Pad left / right to increase / decrease how many upgrades of that item you have.

### Tricks Menu

The Tricks Menu contains some specific functionality to help practice tricks and boss fights. Each trick can be reloaded manually with D-Pad Left, or automatically reloaded if performed successfully. For all boss scenarios, the sword is only enforced when you first select the boss fight. You may select a different sword with the Inventory menu, then reload to try with that sword.

- **Wing Ceremony Cutscene Skip** - When activated, you will be loaded into the save prompt before the Wing Ceremony. The game will keep track of your A presses and evaluate your timing on WCCS. This also kills Link, to make the reload much faster if you succeed.
- **Guay Deathwarp** - When activated, you will be loaded into the Sky as though you just performed the Sky RBW in most early game Skyloft routes, your health will be set to 6 hearts, and the flag for the Fi text near the Faron Pillar will be unset, allowing you to practice the deathwarp with the guay.
- **Keese Yeet** - When activated, you will be loaded into the Earth Temple (positioned for Keese Yeet) and given the Goddess Sword.
- **Extending Blow** - When activated, you will be loaded into Deep Woods (positioned to setup for Extending Blow), and given the B-Wheel, Slingshot, and Goddess Sword. In this scenario, you can see info about the bokoblin you're currently targeting, including the probability of succeeding with a spin attack Extending Blow given your current position.
- **Cutscene Skip Wrong Warp** - When activated, the Gate of Time Skip practice save (from All Dungeons) will be loaded, and the game will eventually teleport you to the Sealed Temple in BiT with 0 health, from which you may perform Death Trick and the subsequent File Dupe. It will evaluate whether or not you missed both tricks' frame windows. Pressing D-Pad right will soft reset and restart the trick sequence.
- **Faron Dive** - When activated, the game will soft reset, and you will eventually be teleported to the Sky, from which you may practice diving to the Faron Pillar. Information on-screen will show your angle relative to the pillar, your current speed, distance, and relative altitude.
- **Lumpy Dive** - See above, but with the backdoor of the Lumpy Pumpkin as your target.
- **Ghirahim 1** - When activated, you will be loaded into the Ghirahim fight in Skyview Temple, with the Goddess Sword and at 2 hearts. Text on-screen will show his health.
- **Scaldera** - When activated, you will be loaded into the Scaldera fight (skipping directly to the spawn cutscene) in Earth Temple, with the Goddess Sword and a full Bomb Bag at 6 hearts. Text on-screen will show his health.
- **Moldarach** - When activated, you will be loaded into the Moldarach fight in Lanayru Mining Facility, with a shield, the Gust Bellows, Clawshots, Slingshot, and _at least_ the Goddess Sword.
- **Koloktos** - When activated, you will be loaded into the Koloktos fight in Ancient Cistern, with the Goddess Sword, Whip, Slingshot, and Bomb Bag at 4 hearts.
- **Tentalus** - When activated, you will be loaded into the Tentalus fight in Sandship, with the Bow and _at least_ the Goddess Longsword.
- **Ghirahim 2** - When activated, you will be loaded into the Ghirahim fight in Fire Sanctuary, at 2 hearts and _at least_ the Goddess White Sword. Text on-screen will show his health.
- **Bilocyte** - When activated, you will be loaded into the Thunderhead with the battle against Levias started, with Spiral Charge, the Bow, and the True Master Sword.
- **Horde** - When activated, you will be loaded into the Horde battle in Hylia's Realm with full health and _at least_ the Master Sword.
- **Ghirahim 3** - When activated, you will be loaded into the Ghirahim battle in Hylia's Realm with full health, a shield, and _at least_ the Goddess Sword.
- **Demise** - When activated, you will be loaded into the Demise fight at the end of the game with full health, a shield, _at least_ the Goddess Sword, _and Clawshots :)_

### Extra Hotkeys

You may enable extra hotkeys in the main menu. When enabled, you may run the following Action Menu commands with a button combination anywhere:

- **C + D-Pad Right**: Save File (gives 1 frame of Guardian potion to indicate a successful save)
- **C + D-Pad Left**: Load Position
- **C + D-Pad Up**: Direct Load File
- **C + D-Pad Down**: Load File

### Frame Advance

You may toggle Frame Advance mode in the main menu. When enabled, the game is paused, and can be unpaused by pressing D-Pad Right. While paused, you may advance the game by 1 frame by tapping the 1 button. Button inputs can be buffered (that is, any buttons you hold while advancing a frame will be treated as though you began tapping them this frame unless they were already held), however, motion control inputs cannot be buffered currently.

## Debug / Extra Features

If you're running from source, you may enable certain experimental / extra features. You will need Python, along with certain dependencies, and devkitPPC to run the assemble scripts.
Run `asm_debug.sh` or manually run `assemble_us.py` or `assemble_jp.py` with the `debug` argument to build the custom REL with extra features. Extra features currently are...

In the Action Menu:

- **Give Item** pulls up a submenu where you may select an item ID and trigger an item get for that item. Not all items work,
  and some have cutscenes associated with them that may cause crashes, so use at your own risk. It is recommended to use the **Inventory Menu** instead.
- **Create Save** saves your currently selected file as though you saved at a Bird Statue, and saves save data. This is useful for making hacked practice saves.

Once you've built the custom rel, just run the patcher like normal (`cargo run --release`), and the custom REL will be copied into `modified-extract`. You may revert to normal features by just running the normal assemble scripts.
