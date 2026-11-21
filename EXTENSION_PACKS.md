# Extension packs

An extension pack is a curated set of extensions for one kind of work.
Zaply ships four packs, defined as simple manifests in the
`extension-packs/` folder of this repository:

| Pack | What it adds |
| --- | --- |
| Python | Framework snippets and tooling for Django, Flask, and FastAPI, plus Cython |
| Rust | Crate version tooling in Cargo.toml, RON and TOML support, Leptos |
| Web | Astro, Angular, Vue, Svelte, Biome, Emmet, snippets, a live server, MDX, GraphQL, Prisma, SCSS, Pug |
| Data Science | R, Julia, SQL, CSV, dbt, Quarto, LaTeX, Mermaid, and a Markdown language server |

The packs add to what is already built into Zaply (Python, Rust,
TypeScript, Jupyter notebooks, and many other languages work out of the
box). A pack is a starting set, not a bundle: every extension in it is
an independent, community-maintained project with its own repository
and license.

## How installation works, honestly

Zaply does not run a hosted extension gallery, so there is no
one-click store install. The extension mechanism that works without a
gallery is the **dev extension**: Zaply builds an extension from source
code in a local folder and keeps it installed. Pack installation is
therefore two steps:

1. Download the sources:

   ```sh
   script/install-extension-pack python
   ```

   The script clones the public extension registry, verifies that
   every extension in the pack exists, and downloads just those
   extensions into `~/.local/share/zaply/extension-pack-sources/`. It
   prints the folder of each downloaded extension.

2. In Zaply, open the Extensions panel, choose **Install Dev
   Extension**, and pick each printed folder. Zaply compiles the
   extension to WebAssembly and installs it. This needs a Rust
   toolchain with the `wasm32-wasip1` target on your machine
   (`rustup target add wasm32-wasip1`).

Installed dev extensions survive restarts and can be rebuilt after an
update (re-run the script, then use **Rebuild Dev Extension** in the
panel) or removed from the same panel.

## Limits

- The final install step is a manual action in the app for each
  extension. There is currently no supported way to install an
  extension from the command line, so the script cannot do it for you.
- Extensions are built from source on your machine, so installing a
  pack takes some CPU time and needs the Rust toolchain above.
- Extension quality, updates, and support belong to each extension's
  own maintainers. The pack manifests in this repository only pin the
  selection, not versions.
- Writing your own extension is covered by `EXTENSION_API.md`.
