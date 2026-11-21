"""Data analysis starter.

Run with: python analysis.py
"""

import csv
from pathlib import Path


def main() -> None:
    data_file = Path("data.csv")
    if not data_file.exists():
        print("Add a data.csv file next to this script to analyse it.")
        return
    with data_file.open(newline="") as handle:
        rows = list(csv.reader(handle))
    print(f"Loaded {len(rows)} rows with {len(rows[0]) if rows else 0} columns")


if __name__ == "__main__":
    main()
