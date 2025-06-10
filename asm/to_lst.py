import yaml
import os


def create_lst(ver: str, dir: str):
    with open(f"original_symbols/{ver}.txt", "r") as f:
        original_symbols: dict = yaml.safe_load(f)

    with open(f"custom_symbols/{ver}.txt", "r") as f:
        custom_symbols: dict = yaml.safe_load(f)

    with open(os.path.join(dir, f"{ver}.lst"), "w") as outf:
        for symbol, addr in original_symbols["main.dol"].items():
            outf.write(f"{hex(addr)[2:]}:{symbol}\n")
        for symbol, addr in custom_symbols["main.dol"].items():
            outf.write(f"{hex(addr)[2:]}:{symbol}\n")
