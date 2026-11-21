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

## Changed files

The rebrand and the total rename touched files across the entire tree:
crates, assets, extensions, script, tooling, and configuration files.
Rather than listing thousands of paths, the categories of change are
recorded in the summary above. This notice and the repository itself
record the modification and its date, as the GPL requires.
