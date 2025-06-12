import sys
import re
import os
from pathlib import Path
import shutil

import disc_riider_py

WIT_PROGRESS_REGEX = re.compile(rb" +([0-9]+)%.*")
CLEAN_NTSC_U_1_00_DOL_HASH = "450a6806f46d59dcf8278db08e06f94865a4b18a"
CLEAN_NTSC_J_1_00_DOL_HASH = "2848bb574bfcbf97f075adc4e0f4692ddd7fd0e8"
VALID_HASHES = [CLEAN_NTSC_U_1_00_DOL_HASH, CLEAN_NTSC_J_1_00_DOL_HASH]

WRONG_VERSION_DOL_HASHES = {
    # "TODO": "US 1.01",
    "30cad7e8a88442b1388867f01bc6461097f4a152": "US 1.02",
    "8f6bf468447d9f10172cc4a472a56e1f526a5cb4": "PAL 1.00",
    # "TODO": "PAL 1.01",
    # "TODO": "PAL 1.02",
}

NOP = lambda *args, **kwargs: None

# currently, only win and linux (both 64bit) are supported
IS_WINDOWS = sys.platform == "win32"


class WrongChecksumException(Exception):
    pass


class ExtractManager:
    def __init__(self, rootpath: Path, japanese: bool):
        self.rootpath = rootpath
        self.japanese = japanese
    
    def extract_is_good(self):
        return self.extract_already_exists() and self.original_dol_already_exists()

    def extract_already_exists(self):
        return (self.extract_path() / "DATA" / "sys" / "main.dol").is_file()

    def extract_path(self):
        return self.rootpath / "extract" / ("JP" if self.japanese else "US")

    def original_dol_path(self):
        return self.rootpath / "original-dol" / ("JP" if self.japanese else "US")

    def is_japanese(self):
        return self.japanese

    def extract_game(self, iso_path, progress_cb=NOP):
        if not self.extract_is_good():
            dest_path = self.extract_path()
            extractor = disc_riider_py.WiiIsoExtractor(iso_path)
            extractor.prepare_extract_section("DATA")
            checksum = bytes(extractor.get_dol_hash("DATA")).hex()
            if checksum not in VALID_HASHES:
                if wrong_version := WRONG_VERSION_DOL_HASHES.get(checksum):
                    raise WrongChecksumException(
                        f"This ISO is {wrong_version}, but the practice patcher only supports NTSC-U 1.00 (North American) or NTSC-J 1.00 (Japanese).",
                    )
                else:
                    raise WrongChecksumException(
                        f"Unrecognized DOL hash, probably bad dump or invalid version: {checksum}",
                    )
            if self.japanese and checksum == CLEAN_NTSC_U_1_00_DOL_HASH:
                raise WrongChecksumException(
                    f"This ISO is the NTSC-U version, but you specified the Japanese version as an argument. Please choose a Japanese copy or run with the `us` argument.",
                )
            elif (not self.japanese) and checksum == CLEAN_NTSC_J_1_00_DOL_HASH:
                raise WrongChecksumException(
                    f"This ISO is the Japanese version, but you specified the NTSC-U version as an argument. Please choose a North American copy or run with the `jp` argument.",
                )
            extractor.extract_to(
                dest_path, lambda x: progress_cb("Extracting files...", x)
            )
            # delete all videos, they take up way too much space
            for hint_vid in (dest_path / "DATA" / "files" / "THP").glob("*.thp"):
                os.remove(str(hint_vid))

            os.makedirs(self.original_dol_path(), exist_ok=True)
            shutil.copy(dest_path / "DATA" / "sys" / "main.dol", self.original_dol_path() / "main.dol")

            if self.is_japanese():
                print("Successfully extracted JP Skyward Sword")
            else:
                print("Successfully extracted US Skyward Sword")

    def original_dol_already_exists(self):
        return (self.original_dol_path() / "main.dol").is_file()

    def repack_game(self, modified_iso_dir: Path, progress_cb=NOP):
        modified_iso_path = modified_iso_dir / (
            "SOUJ01.iso" if self.is_japanese() else "SOUE01.iso"
        )
        if modified_iso_path.is_file():
            modified_iso_path.unlink()
        disc_riider_py.rebuild_from_directory(
            self.extract_path(),
            modified_iso_path,
            lambda x: progress_cb("Writing patched game...", x),
        )
