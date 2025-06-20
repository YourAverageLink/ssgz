# ssgz (name pending)

A ROM hack for *The Legend of Zelda: Skyward Sword* aimed to help out speedrunners with practice and research.

## Downloads / Usage

ssgz is a command-line program that may be run from source or run as an executable from the Releases tab.
It accepts one argument for the version you wish to patch: `us` for the NTSC North American 1.0 version, or `jp` for the Japanese version.
You will need to provide a clean `.iso` file of either of these versions (version code `SOUE01` or `SOUJ01`, respectively).

### Running from Source 
- Install Rust (if necessary): https://rustup.rs/
- Clone this repo

    ``git clone https://github.com/YourAverageLink/ss-practice``
    
    ``cd ss-practice``
- Run (release mode is recommended for optimal performance): `cargo run --release [us | jp]`

### Running an executable:
- Windows:
  - In the command line, navigate to the directory `ss-practice` is installed, and run `ss-practice.exe [us | jp]`
- MacOS / Linux:
  - In the terminal, navigate to the directory `ss-practice` is installed, and run `./ss-practice [us | jp]`


The program will ask you to provide your copy of *Skyward Sword*. Once you select a valid iso, it will begin extracting it to `extract/[version]`, while copying the original `main.dol` file to `original-dol/[version]`. A progress bar will display the extraction progress.
It will then copy a patched dol file over `main.dol` in the extract, copy practice saves, and copy the custom REL file. **Though some large files (namely hint videos / credits videos) are removed to save on space,
note that the extract will still take up more than 2 GB of space.**

Once patching is done, it will ask if you wish to create a new patched iso, and if so, where to put it. A progress bar will display the ISO packing progress. You may play this iso through Dolphin or on console with a USB loader.

## Features / In-Game Usage

While in gz, pressing Z and C simultaneously on the Nunchuck will open up a Practice Menu, with some submenus. A description of each menu item is visible on the bottom of the screen, along with basic control info.

### Display Menu

In the Display Menu, you may toggle whether or not certain information should be passively displayed on-screen
- **Input Viewer** will show any buttons currently pressed, and the directions registered on the Joystick and D-Pad. Note that this display is currently cut off on 4:3 Aspect Ratio
- **Link Pos Viewer** will show Link's x, y, and z coordinates, facing angle, and current movement speed.
- **Scene Flag Viewer** will display the scene flags and temporary flags active in the current scene.
- **Frame Count Viewer** will display the number of in-game frames that have passed since the last load.

### Warp Menu

In the Warp Menu, you may choose a stage you wish to warp to, along with which room, layer, and entrance you wish to use.
Note that some entrances / layers may crash, so use this at your own risk.

### Action Menu

The Action Menu contains some useful miscellaneous functions.
- **Save File** acts like pressing D-Pad Right in the old practice Gecko codes -- it saves your stage, position, flags, etc., which may be loaded by the two following options
- **Load File** will reload the area you saved in with **Save File**, at the last entrance you took on that file.
- **Direct Load File** will reload the file you saved with **Save File** with the coordinates you saved at as well.
- **Kill Link** kills Link (even if you have the **Infinite Health** cheat enabled).
- **RBM Scene Flag** pulls up a submenu where you may select a scene flag to RBM (& commit) in the current area.
- **Enter BiT** will load into Back in Time on Skyloft.

### Cheats Menu

The Cheats Menu contains some useful cheats that will remain active as long as they are toggled here. The cheats include...
- Infinite Health
- Infinite Stamina
- Infinite Slingshot Seeds
- Infinite Bombs
- Infinite Arrows
- Infinite Rupees
- Moon Jump (only active while holding D-Pad Right)

### Practice Saves Menu

The Practice Saves Menu allows you to load practice saves for certain speedrun categories, useful for practicing certain parts of the run. Saves are included for...
- Any% (on the Ghirahim 3 Escape + Fast Faron BiT route, the recommended route at the time of writing)
- All Dungeons (on the Fast Faron BiT CSWW route, the recommended route at the time of writing)
- 100% (on the Imp1 Skip + Fast Faron BiT route, the recommended route at the time of writing)

### Flag Menu

The Flag Menu contains a list of some important story flags for game progress.
Each entry will display `[x]` if it is currently set, and `[]` if it is not. You may press A on each flag to toggle it.

### Inventory Menu

The Inventory Menu contains a list of notable items. Each entry displays the current level of the item you have (for instance, "Goddess Sword" on the Sword row if that's what you have).
For each item, you may use D-Pad left / right to increase / decrease how many upgrades of that item you have.

### Tricks Menu

The Tricks Menu contains some specific functionality to help practice tricks. Each trick can be reloaded manually with D-Pad Left, or automatically reloaded if performed successfully.

- **Wing Ceremony Cutscene Skip** - When activated, you will be loaded into the save prompt before the Wing Ceremony. The game will keep track of your A presses and evaluate your timing on WCCS. This also kills Link, to make the reload much faster if you succeed.
- **Guay Deathwarp** - When activated, you will be loaded into the Sky as though you just performed the Sky RBW in most early game Skyloft routes, your health will be set to 6 hearts, and the flag for the Fi text near the Faron Pillar will be unset, allowing you to practice the deathwarp with the guay.
- **Keese Yeet** - When activated, you will be loaded into the Earth Temple (positioned for Keese Yeet) and given the Goddess Sword.
- **Extending Blow** - When activated, you will be loaded into Deep Woods (positioned to setup for Extending Blow), and given the B-Wheel, Slingshot, and Goddess Sword.

## Debug / Extra Features

If you're running from source, you may enable certain experimental / extra features. You will need Python, along with certain dependencies, and devkitPPC to run the assemble scripts.
Run `asm_debug.sh` or manually run `assemble_us.py` or `assemble_jp.py` with the `debug` argument to build the custom REL with extra features. Extra features currently are...

In the Action Menu:
- **Give Item** pulls up a submenu where you may select an item ID and trigger an item get for that item. Not all items work,
and some have cutscenes associated with them that may cause crashes, so use at your own risk. It is recommended to use the **Inventory Menu** instead.
- **Create Save** saves your currently selected file as though you saved at a Bird Statue, and saves save data. This is useful for making hacked practice saves.

Once you've built the custom rel, just run the patcher like normal (`cargo run --release`), and the custom REL will be copied into `modified-extract`. You may revert to normal features by just running the normal assemble scripts.
