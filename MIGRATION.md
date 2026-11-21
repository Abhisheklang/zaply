# Moving to Zaply

Zaply never reads another editor's configuration folders on its own.
Every import below is an explicit action you run, which reads the file
you point it at and writes Zaply configuration. Nothing happens in the
background and nothing is uploaded anywhere.

## From VS Code

### Settings

Run **Import VS Code Settings** (offered during first-run onboarding,
and available afterwards from the command palette). Zaply reads your
VS Code `settings.json` and converts what it understands: editor
preferences such as font, tab size, line length, wrapping, and format
on save, plus language-specific settings for common languages. It also
switches your base keymap to the VS Code layout, so the shortcuts you
know keep working. Settings with no Zaply equivalent are skipped, and
your existing Zaply settings are backed up before anything is written.

Cursor settings can be imported the same way with **Import Cursor
Settings**.

### Keybindings

Your custom VS Code keybindings live in a separate file, so they have
their own action: **Import VS Code Keybindings**. Point it at your
`keybindings.json` (in VS Code: Command Palette, "Preferences: Open
Keyboard Shortcuts (JSON)"). The converter handles the common editor
commands (save, close editor, quick open, command palette, find in
files, terminal toggle, clipboard, undo/redo, comment, rename, format,
completions, go to line, back/forward navigation) and maps VS Code
`when` clauses for the editor and terminal to Zaply contexts. The
result is merged into your Zaply keymap file, and a message afterwards
tells you how many entries were imported and how many were skipped
because they have no Zaply equivalent (extension commands, and
bindings guarded by `when` clauses Zaply cannot express). Removal
entries (commands starting with `-`) are skipped.

## Between Zaply installs

**Export Settings** (command palette) writes your settings and keymap
into a single `zaply-settings-export.json` file in a folder you pick.
Carry it to another machine and run **Import Settings** there. Before
replacing anything, the import saves your current files next to the
originals with a `.zaply-backup` suffix, so you can undo it by hand.
Zaply reloads settings and keymap files automatically when they
change, so the import takes effect immediately.

The export contains only your settings and keymap. It does not contain
API keys (those stay in your system keychain) or any project files.

## Project templates

**New Project from Template** (command palette) creates a starter
project in an empty folder you pick:

- **Python package**: a `pyproject.toml`, a `main.py`, and a README
- **Static website**: `index.html`, `style.css`, and `script.js` wired
  together
- **Rust binary**: a `Cargo.toml` and `src/main.rs`
- **Data science**: an analysis script, a starter Jupyter notebook,
  and a `requirements.txt`

The action refuses to write into a folder where any of the template's
files already exist, so it cannot overwrite your work.

## Per-project settings and indexing controls

Settings that should only apply to one project go in
`.zaply/settings.json` in the project root. They override your global
settings for that project. This is also where you control what Zaply
scans and indexes for a project:

```json
{
  "file_scan_exclusions": [
    "**/node_modules",
    "**/generated-clients",
    "..."
  ]
}
```

Files matching `file_scan_exclusions` are skipped by the file finder,
project search, and the agent's codebase context. The `"..."` entry
means "keep the built-in defaults as well". To switch scanning off for
a whole project, exclude `"**"` (the project still opens; it just has
no searchable file index). Remove the file, or the entries, to go back
to the defaults. The **Large Monorepo** profile described in
`BENCHMARKS.md` writes a starter set of these exclusions globally.
