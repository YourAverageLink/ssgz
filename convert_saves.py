from io import BytesIO
from pathlib import Path
import shutil
import glob

def write_magic_str(data, offset, new_string, max_length):
    # Writes a fixed-length string that does not have to end with a null byte.
    # This is for magic file format identifiers.

    str_len = len(new_string)
    if str_len > max_length:
        raise Exception(
            "String %s is too long (max length 0x%X)." % (new_string, max_length)
        )

    padding_length = max_length - str_len
    null_padding = b"\x00" * padding_length
    new_value = new_string.encode("shift_jis") + null_padding

    data.seek(offset)
    data.write(new_value)


def copy_jp_to_us(jp_path: Path, us_path: Path):
    shutil.copy(jp_path, us_path)
    src_bytes = BytesIO(us_path.read_bytes())
    write_magic_str(src_bytes, 0, "SOUE", 4)
    src_bytes.seek(0)
    # print(src_bytes.read(4))
    us_path.write_bytes(src_bytes.getvalue())


# Copies from JP practice saves dir to US, replacing the SOUJ magic string with SOUE
if __name__ == "__main__":
    if Path("practice-saves/US/saves").exists():
        shutil.rmtree("practice-saves/US/saves")
    shutil.copytree("practice-saves/JP/saves", "practice-saves/US/saves")

    jp_paths = [
        path for path in glob.glob("practice-saves/JP/**/wiiking2.sav", recursive=True)
    ]
    for path in jp_paths:
        print(f"Copying {path}")
        copy_jp_to_us(Path(path), Path(path.replace("/JP/", "/US/")))
    # copy_jp_to_us(p.src_path, p.dest_path)
