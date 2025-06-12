from pathlib import Path
from collections import OrderedDict, defaultdict
import argparse

import yaml
import sys
from io import BytesIO
import shutil
from extractmanager import ExtractManager

import tkinter as tk
from tkinter import filedialog

from paths import ROOT_PATH

from sslib import U8File
from sslib.utils import write_bytes_create_dirs
from sslib.dol import DOL
from sslib.rel import REL

from asm.patcher import apply_dol_patch


class GamePatcher:
    def __init__(
        self,
        extract_path,
        original_dol_path,
        is_japanese,
    ):
        self.extract_path = extract_path
        self.dol_path = original_dol_path
        self.is_japanese = is_japanese

    def do_all_gamepatches(self):
        # Currently, only main.dol is patched directly
        self.load_base_patches()
        self.do_dol_patch()
        # self.do_rel_patch()

    def load_base_patches(self):
        # assembly patches
        self.all_asm_patches = defaultdict(OrderedDict)

        # for asm, custom symbols
        with (
            ROOT_PATH
            / "asm"
            / "custom_symbols"
            / ("jp.txt" if self.is_japanese else "us.txt")
        ).open("r") as f:
            self.custom_symbols = yaml.safe_load(f)
        self.main_custom_symbols = self.custom_symbols.get("main.dol", {})
        with (
            ROOT_PATH
            / "asm"
            / "original_symbols"
            / ("jp.txt" if self.is_japanese else "us.txt")
        ).open("r") as f:
            self.original_symbols = yaml.safe_load(f)
        self.main_original_symbols = self.original_symbols.get("main.dol", {})

        # for asm, free space start offset
        with (
            ROOT_PATH
            / "asm"
            / "free_space_start_offsets"
            / ("jp.txt" if self.is_japanese else "us.txt")
        ).open("r") as f:
            self.free_space_start_offsets = yaml.safe_load(f)
        self.add_asm_patch("ss_necessary")

    def add_asm_patch(self, name):
        with (
            ROOT_PATH
            / "asm"
            / "patch_diffs"
            / ("jp" if self.is_japanese else "us")
            / f"{name}_diff.txt"
        ).open("r") as f:
            asm_patch_file_data = yaml.safe_load(f)
        for exec_file, patches in asm_patch_file_data.items():
            self.all_asm_patches[exec_file].update(patches)

    def do_dol_patch(self):
        # patch main.dol
        print("Patching main.dol...")
        dol_bytes = BytesIO((self.dol_path / "main.dol").read_bytes())
        dol = DOL()
        dol.read(dol_bytes)
        apply_dol_patch(self, dol, self.all_asm_patches["main.dol"], self.is_japanese)

        dol.save_changes()
        write_bytes_create_dirs(
            self.extract_path / "DATA" / "sys" / "main.dol",
            dol_bytes.getbuffer(),
        )

    # def do_rel_patch(self):
    #     rel_arc = U8File.parse_u8(
    #         BytesIO(
    #             (self.extract_path / "DATA" / "files" / "rels.arc").read_bytes()
    #         )
    #     )
    #     rel_modified = False
    #     for file, codepatches in self.all_asm_patches.items():
    #         if file == "main.dol":  # main.dol
    #             continue
    #         rel_data = BytesIO(rel_arc.get_file_data(f"rels/{file}"))
    #         if rel_data is None:
    #             print(f"ERROR: rel {file} not found!")
    #             continue
    #         rel = REL()
    #         rel.read(rel_data)
    #         apply_rel_patch(self, rel, file, codepatches)
    #         rel.save_changes()
    #         rel_arc.set_file_data(f"rels/{file}", rel_data.getbuffer())
    #         rel_modified = True
    #     if rel_modified:
    #         print("Patching rels...")
    #         rel_data = rel_arc.to_buffer()
    #         write_bytes_create_dirs(
    #             self.modified_extract_path / "DATA" / "files" / "rels.arc",
    #             rel_data,
    #         )

    def copy_practice_saves(self):
        print("Copying practice saves...")
        src_path = (
            ROOT_PATH
            / "practice-saves"
            / ("JP" if self.is_japanese else "US")
            / "saves"
        )
        dest_path = self.extract_path / "DATA" / "files" / "saves"
        if dest_path.is_dir():
            shutil.rmtree(dest_path)
        shutil.copytree(src_path, dest_path)

    def copy_custom_rel(self):
        print("Copying custom REL file...")
        src_path = (
            ROOT_PATH
            / "custom-rel"
            / ("JP" if self.is_japanese else "US")
            / "customNP.rel"
        )
        dest_path = self.extract_path / "DATA" / "files" / "rels" / "customNP.rel"
        shutil.copyfile(src_path, dest_path)


parser = argparse.ArgumentParser(
    prog=sys.argv[0],
    description="Patches a US or JP copy of Skyward Sword with useful speedrunning practice features",
    epilog="Text at the bottom of help",
)

parser.add_argument(
    "version",
    choices=["us", "jp"],
    help="Which version of the game you want to patch. Note that only their respective 1.0 versions are supported.",
)

if __name__ == "__main__":
    args = parser.parse_args()
    version = args.version.lower().strip()

    if version not in ["us", "jp"]:
        print("Version must be 'us' or 'jp'")
    else:
        japanese = version == "jp"
        extract = ExtractManager(Path(".").resolve(), japanese)
        extract_exists = extract.extract_already_exists()
        dol_exists = extract.original_dol_already_exists()
        if not (extract_exists and dol_exists):
            if not extract_exists:
                print(
                    f"To create a practice rom for the {'JP' if japanese else 'NTSC US'} version, a clean copy of the {'JP' if japanese else 'NTSC US'} version is needed."
                )
            elif not dol_exists:
                print(
                    "The original copy of `main.dol` is missing. It is recommended to redo an extract of your copy of Skyward Sword for patches to work properly."
                )
            root = tk.Tk()
            root.withdraw()
            file_path = filedialog.askopenfilename(
                defaultextension=".iso",
                filetypes=[("Wii ROMs", ".iso")],
                title="Select a .iso file.",
            )
            root.destroy()
            print("Extracting game files, this may take some time...")
            extract.extract_game(file_path)
            print("Extracting done")

        # Ensure the extract worked properly
        if extract.extract_is_good():
            if japanese := extract.is_japanese():
                print("Patching Japanese version")
            else:
                print("Patching North American Version")
            patcher = GamePatcher(
                extract.extract_path(), extract.original_dol_path(), japanese
            )
            patcher.do_all_gamepatches()
            patcher.copy_practice_saves()
            patcher.copy_custom_rel()
            user_wants_iso = input(
                "Patching done, want to write an output iso? (y or n): "
            )
            if user_wants_iso.strip().lower() == "y":
                root = tk.Tk()
                root.withdraw()
                output_dir = Path(
                    filedialog.askdirectory(
                        title="Select a directory to output the iso to."
                    )
                )
                root.destroy()
                if output_dir.exists():
                    print("Writing patched iso, this may take some time...")
                    extract.repack_game(output_dir)
                else:
                    print("Error when selecting output directory!")

            print(
                "All done, happy speedrunning! Press Z and C simultaneously to access practice menus!"
            )
