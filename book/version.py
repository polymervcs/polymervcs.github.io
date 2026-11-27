#!/usr/bin/env python3
"""An mdBook preprocessor (see book.toml) that replaces {{version}} in the docs with the newest PolymerVCS release:
the version in the site's Cargo.toml, which the downloads page uses too.

mdBook runs it with `supports <renderer>` to ask whether it works with a renderer, and then with no arguments, and
[context, book] as JSON on stdin, to get the changed book on stdout.
"""

import json
import re
import sys
import tomllib
from pathlib import Path

CARGO_TOML = Path(__file__).resolve().parent.parent / "Cargo.toml"
PLACEHOLDER = re.compile(r"\{\{\s*version\s*\}\}")


def fill_in(items, version):
    for item in items:
        # The other items are part titles and separators.
        if isinstance(item, dict) and "Chapter" in item:
            chapter = item["Chapter"]
            chapter["content"] = PLACEHOLDER.sub(version, chapter["content"])
            fill_in(chapter["sub_items"], version)


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "supports":
        return

    version = tomllib.loads(CARGO_TOML.read_text(encoding="utf-8"))["package"]["version"]
    _context, book = json.load(sys.stdin)
    fill_in(book["items"], version)
    json.dump(book, sys.stdout)


if __name__ == "__main__":
    main()
