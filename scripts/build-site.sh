#!/usr/bin/env bash
# Pre-render the site with Dioxus SSG and stage it in ./dist for GitHub Pages.
set -euo pipefail

cd "$(dirname "$0")/.."

CRATE_NAME="polymervcs-site"
PUBLIC_DIR="target/dx/${CRATE_NAME}/release/web/public"
OUT_DIR="dist"

# dx never prunes old hashed assets from this folder, so start fresh.
rm -rf "$PUBLIC_DIR"

# dx regenerates this from tailwind.css but ignores Tailwind errors. Deleting it
# first means a failed Tailwind run fails the build at `asset!` instead of
# shipping stale CSS.
rm -f assets/tailwind.css

# Install daisyUI, and everything else in package.json, such as allo-font
npm i -D daisyui@latest

# Use `dx build`, not `dx bundle`: as of dx 0.7.10, `bundle` accepts --ssg but
# never pre-renders the routes.
# --force-sequential builds the client before the server. Otherwise they build
# in parallel, and if the server finishes first the client build overwrites
# the pre-rendered index.html with an empty shell.
dx build --web --ssg --release --force-sequential

rm -rf "$OUT_DIR"
cp -r "$PUBLIC_DIR" "$OUT_DIR"

# GitHub Pages serves /404.html for any missing path. The page was pre-rendered
# from the /404 route, so it hydrates correctly wherever it's served.
if [[ -f "$OUT_DIR/404/index.html" ]]; then
  mv "$OUT_DIR/404/index.html" "$OUT_DIR/404.html"
  rmdir "$OUT_DIR/404" 2>/dev/null || true
fi

# Custom domain, and no Jekyll processing if this folder is ever published
# from a branch instead of through GitHub Actions.
cp CNAME "$OUT_DIR/CNAME"
touch "$OUT_DIR/.nojekyll"

echo "Static site written to $OUT_DIR/"
