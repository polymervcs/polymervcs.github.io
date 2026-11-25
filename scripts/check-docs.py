#!/usr/bin/env python3
"""Checks the docs built into <site>/docs/ by scripts/build-site.sh, as mdBook doesn't:

- Every page in docs/ is in docs/SUMMARY.md. mdBook only builds the pages it lists, so a page that's left out has no
  HTML page.
- Every link in a page goes to a file in the site that exists, and to a heading on that page that exists. Links to the
  rest of the site, such as /downloads, are checked too, so this needs the whole site, not just the docs.

It reads the built HTML rather than the Markdown, so the links and heading ids are the ones mdBook made.

Usage: scripts/check-docs.py [site folder, which defaults to dist]
"""

import os
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

DOCS = Path(__file__).resolve().parent.parent / "docs"


class Page(HTMLParser):
    """The ids in a page, and the links in its <main>, which holds the page's Markdown."""

    def __init__(self, path):
        super().__init__()
        self.ids = set()
        self.links = []
        self.in_main = False
        self.feed(path.read_text(encoding="utf-8"))

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if attrs.get("id"):
            self.ids.add(attrs["id"])
        if tag == "main":
            self.in_main = True
        # Not the links mdBook makes from each heading to itself
        elif tag == "a" and self.in_main and attrs.get("href") and "header" not in (attrs.get("class") or "").split():
            self.links.append(attrs["href"])

    def handle_endtag(self, tag):
        if tag == "main":
            self.in_main = False


def served_file(site, page, path):
    """The file GitHub Pages serves for a link's path, or None if there isn't one."""
    target = site / path.lstrip("/") if path.startswith("/") else page.parent / path
    target = Path(os.path.normpath(target))
    # /docs/ serves docs/index.html, and /docs/user/install serves docs/user/install.html.
    for candidate in (target, target / "index.html", target.with_name(target.name + ".html")):
        if candidate.is_file():
            return candidate
    return None


def main():
    site = Path(sys.argv[1] if len(sys.argv) > 1 else "dist")
    built = site / "docs"
    if not (built / "index.html").is_file():
        sys.exit(f"There are no docs in {built}. Build the site first, with scripts/build-site.sh.")

    problems = []
    parsed = {}

    def parse(path):
        if path not in parsed:
            parsed[path] = Page(path)
        return parsed[path]

    sources = sorted(path.relative_to(DOCS) for path in DOCS.rglob("*.md") if path.name != "SUMMARY.md")
    links = 0
    for source in sources:
        name = f"docs/{source.as_posix()}"
        page = built / source.with_suffix(".html")
        if not page.is_file():
            problems.append(f"{name} isn't in docs/SUMMARY.md, so it isn't built")
            continue

        for href in parse(page).links:
            url = urlsplit(href)
            if url.scheme or url.netloc:
                # Another site, or mailto:
                continue
            links += 1
            path, heading = unquote(url.path), unquote(url.fragment)
            target = served_file(site, page, path) if path else page
            if target is None:
                problems.append(f"{name} links to {href}, which doesn't exist")
            elif heading and target.suffix == ".html" and heading not in parse(target).ids:
                problems.append(f"{name} links to {href}, but that page has no #{heading} heading")

    if problems:
        print("Problems in the docs:", *problems, sep="\n  ", file=sys.stderr)
        sys.exit(1)
    print(f"Checked the docs: {len(sources)} pages, all in docs/SUMMARY.md, and {links} links, which all work.")


if __name__ == "__main__":
    main()
