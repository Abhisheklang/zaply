# Modifications to the upstream codebase

Zaply is a fork of the Zed code editor (upstream version 1.29.3, upstream
commit d01b134cddf5e4f71964deb2d6f81cdb38fcf1e7). In line with the GNU
General Public License, this file records the changes made in the fork.
All changes are dated 2026-11-21 and are summarised here:

- Product rebrand on every surface: application name, menus, welcome
  and onboarding screens, settings descriptions, desktop files, store
  metadata, icons, and the command-line interface now say Zaply.
- Total rename: after the rebrand, every remaining upstream name was
  renamed or removed across the whole tree, including crate and package
  names, internal identifiers, environment variables (only ZAPLY_*
  names remain), the URL scheme (zaply:// only), IPC and socket names,
  log strings, generated-file templates, and file and directory names.
  The upstream name survives only in licence and attribution contexts
  (the LICENSE and NOTICE files, copyright headers, and this file and
  the README attribution note).
- Compatibility reads removed: per-project settings are read and
  written under `.zaply/` only, and the legacy URL scheme alias and
  legacy environment-variable aliases were removed.
- Hosted provider deleted: the hosted AI provider, its settings block,
  its model-provider implementation, and the hosted web-search provider
  were deleted outright.
- Telemetry removed: the code that sent usage data and crash reports to
  the upstream servers was deleted. Zaply sends no telemetry at all.
- Auto-update reworked: the editor never polls or downloads from the
  upstream update service. Updates are published as GitHub releases and
  installed manually.
- Hosted services removed: account sign-in, hosted billing and trials,
  hosted edit predictions, and the hosted collaboration service are not
  part of Zaply. Bring-your-own-key AI providers and local models are
  unchanged.
- Links repointed: in-app links that pointed at the upstream website now
  point at this repository, or were removed where no equivalent exists.
- Upstream documentation and hosted-service material removed: the
  upstream product documentation folder, the hosted service's legal
  documents, and the docs-site and hosted-service infrastructure
  folders were deleted.
- Packaging added: a GitHub Actions workflow builds Zaply for Linux and
  packages it as a .deb, an AppImage, and a tar.gz. The upstream
  workflows were removed.
- README.md was rewritten as a Zaply product document, and install.sh
  was added.

## Changes after 1.0 (the 3.0 development pass, also dated 2026-11-21)

- Agent safety layer: the shipped default settings now require
  confirmation before destructive terminal commands and file deletion,
  never allow the agent to edit secret files (`.env`, key and
  certificate files), and allow read-only inspection commands without a
  prompt. A persistent audit log of agent tool calls and permission
  decisions is kept in the data directory and viewable in the app
  (Open Agent Audit Log). Python defaults were tuned (4-space tabs,
  88-column lines, format on save).
- Linux build widened: the Linux packages are now built inside an
  Ubuntu 20.04 container (glibc 2.31 baseline) instead of on the
  current runner image, and the built binary's maximum required GLIBC
  version is printed in the build log.
- Windows and macOS builds added: GitHub Actions workflows build an
  Inno Setup installer for Windows 10 and newer and DMGs for macOS 12
  and newer (deployment target verified from the built binary).
  script/bundle-windows.ps1 now locates Visual Studio with vswhere
  instead of a hardcoded edition path.
- Migration tools: a VS Code keybindings converter (settings crate)
  and an Import VS Code Keybindings action; Export Settings and Import
  Settings actions that move settings and keymap between installs as a
  single file, keeping backups; a New Project from Template action and
  four bundled templates under assets/templates/; MIGRATION.md
  documents the flows.
- AI depth: the agent profiles are presented as Ask, Edit, and Agent;
  token usage reported by providers is accumulated locally per
  provider, model, and day (agent crate) and shown by an Open API Usage
  action. The counts never leave the device.
- Collaboration: the collaboration panel no longer offers sign-in to a
  hosted service that does not exist; it explains self-hosting instead.
  The collab server gained a CI workflow that builds it and verifies
  its health check against PostgreSQL, and COLLABORATION.md documents
  running it and pointing the editor at it via the server_url setting.
- Themes: a Create Custom Theme action starts a user theme from the
  bundled One family in the themes directory, which is watched for live
  preview.
- Performance: the default file scan exclusions now cover dependency
  and build-output folders; script/zaply-benchmarks measures startup,
  large-file open, and project search; BENCHMARKS.md records the
  measured results; Enable Low-End Hardware Mode and Enable Large
  Monorepo Mode actions apply named settings profiles.
- Extension system removed (2026-11-22): Zaply briefly shipped a store
  client pointed at Zed's public extension API (with a "Powered by Zed"
  label in the Extensions view) earlier on 2026-11-22. Testing then
  proved that code-bearing store extensions could not load at all,
  because the v1.0 rename changed the extension ABI identifiers the
  store packages carry. Rather than ship a half-working store, the
  whole extension system was removed the same day: the store client
  and its API base, the Extensions UI and every entry point to it, the
  wasm extension host, dev-extension installs, extension packs, the
  extension API surface, and the collab server's extension catalog
  endpoints. No extension-related network code remains. The language
  data of the bundled first-party set (HTML, GLSL, Proto, and the Java
  family: Java, Gradle, Gradle KTS, and Properties) was converted to
  compiled-in built-in language support before the removal, so every
  language that worked before still works, offline, with no install
  step. Their extension language servers were already non-functional
  via the ABI break, so no working capability was lost.

## Changed files

The rebrand and the total rename touched files across the entire tree:
crates, assets, script, tooling, and configuration files.
Rather than listing thousands of paths, the categories of change are
recorded in the summary above. This notice and the repository itself
record the modification and its date, as the GPL requires.
