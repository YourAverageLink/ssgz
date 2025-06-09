import yaml


def load(ver: str):
    with open(f"{ver}.txt", "r") as f:
        original_symbols: dict = yaml.safe_load(f)

    with open(f"{ver}.lst", "w") as outf:
        for symbol, addr in original_symbols["main.dol"].items():
            outf.write(f"{hex(addr)[2:]}:{symbol}\n")


if __name__ == "__main__":
    load("us")
    load("jp")
