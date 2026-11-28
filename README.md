# polymervcs.github.io

Source for [polymervcs.com](https://polymervcs.com). It's a [Dioxus](https://dioxuslabs.com) 0.7 app that is pre-rendered to static HTML with Dioxus static site generation (SSG) and published to GitHub Pages.

## Requirements

- Rust (stable) with the WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Dioxus CLI 0.7.10: `cargo binstall dioxus-cli@0.7.10` (or `cargo install dioxus-cli@0.7.10 --locked`)
- mdBook 0.5.4, for the docs: `cargo binstall mdbook@0.5.4` (or `cargo install mdbook@0.5.4 --locked`)
- Python 3.11 or later, which fills in the version in the docs and checks their links

## Styling

The site uses [Tailwind CSS](https://tailwindcss.com) v4, [daisyUI](https://daisyui.com) 5 and the [typography plugin](https://github.com/tailwindlabs/tailwindcss-typography) (`prose` classes), all without Node. The typography plugin is built into the standalone Tailwind CLI, so it has no file of its own. `tailwind.css` is the input. dx finds it automatically, downloads the Tailwind CLI, and writes `assets/tailwind.css` (git-ignored) on every build. daisyUI is the standalone plugin file `vendor/daisyui.mjs` (v5.8.1). To upgrade it, replace that file with `daisyui.mjs` from a newer [daisyUI release](https://github.com/saadeghi/daisyui/releases) and update the version in `tailwind.css`.

The site font is [Allo](https://allo.joshpuckett.me), from the `allo-font` npm package. `src/main.rs` bundles `node_modules/allo-font` as a folder asset, which dx copies unhashed to `/assets/allo-font/`, and links its `allo-friendly.css` (roundness fixed at the default 50, less than half the size of `allo.css`'s font). `tailwind.css` puts "Allo" first in `--font-sans`, which Tailwind applies to the whole page. Run `npm install` before building, or the `asset!` for the font fails to compile.

The theme button in the top corner of every page (`src/components/theme_button.rs`) switches between Auto, which follows the system's theme, daisyUI's light theme and its dark theme. It sets `data-theme` on `<html>`, or removes it for Auto. It saves the choice in `localStorage` the way the docs' theme picker does, as `mdbook-theme` (`light`, `coal` for dark, or nothing for Auto), so the docs and the rest of the site share it. `THEME_SCRIPT`, inline in each page's `<head>`, applies the saved theme before the page is shown, and the `theme-auto`, `theme-light` and `theme-dark` variants in `tailwind.css` pick the button's icon from `data-theme`, so it's right before the WebAssembly loads.

## Develop

```bash
npm install
dx serve --web
```

`dx serve` doesn't serve the docs at `/docs`. To work on them, run `mdbook serve book` (see [Documentation](#documentation)).

## Build the static site

```bash
./scripts/build-site.sh
```

This runs `dx build --web --ssg --release --force-sequential`, copies the output into `dist/`, builds the docs into `dist/docs/`, and checks the docs' links. The script explains each flag in comments: in dx 0.7.10, `dx bundle` silently skips SSG, and parallel builds can overwrite the pre-rendered `index.html`. To preview the result:

```bash
python3 -m http.server -d dist 8080
```

## How it works

- Each route's page is in `src/views/`, and components shared between pages are in `src/components/`.
- `src/main.rs` defines the routes. The `static_routes` server function returns every route without dynamic segments, and `dx bundle --ssg` renders each one to `<route>/index.html`. The server build only runs at build time. The deployed site is plain static files.
- In the browser, the WebAssembly bundle hydrates the pre-rendered HTML, so links switch pages on the client side.
- The `/404` route is moved to `dist/404.html`, which GitHub Pages serves for unknown paths.
- `CNAME` is copied into `dist/` and `.nojekyll` is added.
- The `version` in this repository's `Cargo.toml` is the newest release, such as `0.1.0`. Update it on every release, to the plain version from the PolymerVCS workspace `Cargo.toml`, without the build number. `src/main.rs` reads it as `LATEST_VERSION`, and the docs fill it in too (see [Documentation](#documentation)). The downloads page at `/downloads` puts it in the file names it lists, and links each name to `https://github.com/polymervcs/polymervcs/releases/download/v<version>/<name>`. So tag each release `v<version>`, such as `v0.1.0`, and upload its files under the names the page shows.
- When `dx build --ssg` starts the server, the server writes `LATEST_VERSION` as a JSON string, such as `"0.1.0"`, to `api/v1/releases/latest.json`. pvcs's update check fetches it from `https://polymervcs.com/api/v1/releases/latest.json`.

## Documentation

The docs at `/docs` are an [mdBook](https://rust-lang.github.io/mdBook/): Markdown files in `docs/`, built with `book/book.toml` and the theme in `book/theme/`. `scripts/build-site.sh` builds them into `dist/docs/`. To preview them on their own while you write, with live reload:

```bash
mdbook serve book
```

That serves them at the root of `http://localhost:3000`, rather than at `/docs`, so links to the rest of the site, such as `/downloads`, don't work there. `mdbook build book` writes them to `book/build/` (git-ignored).

- `docs/SUMMARY.md` lists every page in sidebar order, in [mdBook's format](https://rust-lang.github.io/mdBook/format/summary.html): after its `# Summary` title, a `# ` line starts a section, and a `- [Title](path.md)` or `[Title](path.md)` line adds a page. mdBook only builds the pages it lists. `index.md` is served at `/docs/`, and `path/name.md` at `/docs/path/name.html`.
- Pages link to each other by relative `.md` paths, such as `[Configuration](../admin/configuration.md#email)`, which also work on GitHub. mdBook rewrites them to `.html`.
- `{{version}}` in a page becomes the newest release, the `version` in `Cargo.toml`, such as in the install pages' file names: `polymervcs-{{version}}-universal.pkg`. The preprocessor `book/version.py` fills it in, so `mdbook serve book` shows it too.
- mdBook doesn't check links, so after building the docs, `scripts/build-site.sh` runs `scripts/check-docs.py`. It fails the build if a page in `docs/` isn't in `SUMMARY.md`, a link in the docs goes to a page or heading that doesn't exist, including links to the rest of the site, such as `/downloads`, or a page still has a placeholder in `{{ }}`, such as a misspelled `{{version}}`. To check changes to the docs without the whole build, rebuild just the docs into an existing `dist/`, then run the check:

  ```bash
  mdbook build book --dest-dir dist/docs
  ```

  ```bash
  python3 scripts/check-docs.py
  ```
- Headings get ids from their text, such as `#set-up-email` for "Set up email".
- `<code class="copy">pvcs update</code>` in a page is inline code with a copy button after it, like the site's `CopyButton`. `book/theme/copy-button.js` adds the button. On GitHub, and without JavaScript, it's plain inline code.
- GitHub-style alerts (`> [!NOTE]`, `[!TIP]`, `[!IMPORTANT]`, `[!WARNING]` and `[!CAUTION]`) are styled as daisyUI soft alerts.
- The theme is in the site's style: the same font, daisyUI's light and dark colors, Tailwind's `prose` text styles, and the site's footer, plus mdBook's search, a light and dark theme picker, and copy buttons on code. Its CSS copies the Tailwind and daisyUI values by hand, so a change to the site's look, such as in `tailwind.css` or the footer in `src/views/shell.rs`, needs the same change in `book/theme/css/`.
- `book/theme/index.hbs` is based on mdBook 0.5's own template, as its scripts depend on the ids in it. When upgrading mdBook, check the theme still works, and change `MDBOOK_VERSION` in the workflow.
- `book/theme/fonts/` has a copy of the Allo font from `node_modules/allo-font`, as mdBook can only use files in the theme folder.
- Links from the rest of the site to the docs are plain `a` elements, such as `a { href: "/docs/" }`, not router `Link`s, as the docs aren't Dioxus routes.

## Deploy

`.github/workflows/deploy.yml` builds and deploys on every push to `main`, on GitHub-hosted runners. In the repository's **Settings → Pages**, **Source** must be **GitHub Actions**. The custom domain (`polymervcs.com`) is also configured there.

When upgrading Dioxus, change the `dioxus` version in `Cargo.toml` and `DIOXUS_CLI_VERSION` in the workflow together. The workflow also installs the mdBook version in `MDBOOK_VERSION`, which should match the one in [Requirements](#requirements).
