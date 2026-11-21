# The Zaply extension API

Zaply extensions are WebAssembly components that add languages,
language servers, debuggers, themes, slash commands, and AI context
servers to the editor. This document describes the API surface as it
exists in this repository today. The normative definition is the WIT
(WebAssembly Interface Types) in `crates/extension_api/wit/`; the Rust
crate `zaply_extension_api` (in `crates/extension_api`) wraps it.

Current API version: **0.8.0**. Versioned WIT snapshots live in
`crates/extension_api/wit/since_v*/`; an extension built against an
older snapshot keeps working, and new capabilities are added in new
snapshots rather than by changing old ones.

## The manifest: extension.toml

Every extension has an `extension.toml` at its root:

```toml
id = "my-extension"
name = "My Extension"
description = "What it does."
version = "0.1.0"
schema_version = 1
authors = ["Your Name"]
repository = "https://example.com/my-extension"

[grammars.my-language]
repository = "https://github.com/example/tree-sitter-my-language"
commit = "0123456789abcdef0123456789abcdef01234567"

[language_servers.my-language]
name = "My Language Server"
language = "My Language"

[[capabilities]]
kind = "process:exec"
command = "my-language-server"
```

An extension with no Rust code is valid: themes, icon themes,
grammars, language definitions (in a `languages/` folder), and
snippets can all be shipped as manifest plus data files. See
`extensions/` in this repository for examples.

## Capabilities

Extensions are sandboxed. Anything an extension wants to do beyond
pure computation is declared in `[[capabilities]]` and granted by the
host:

- `process:exec` for specific commands (used to start language
  servers, debug adapters, and context servers)
- HTTP requests through the host's HTTP client (the extension never
  gets raw sockets)
- Downloading release assets from GitHub for server binaries

## The Extension trait (Rust)

Code extensions implement `zaply_extension_api::Extension` and
register with `register_extension!`. All methods have default
implementations; implement only what your extension provides.

Language servers:

- `language_server_command`: how to start the server for a worktree
  (the host downloads and caches the binary you describe)
- `language_server_initialization_options` and
  `language_server_workspace_configuration`: JSON sent to the server,
  with optional `*_schema` methods describing it
- `language_server_additional_initialization_options` and
  `language_server_additional_workspace_configuration`: per-server
  extras when one extension provides several servers
- `label_for_completion` and `label_for_symbol`: custom rendering of
  completion items and symbols

AI and agent features:

- `run_slash_command` and `complete_slash_command_argument`: slash
  commands in the agent panel
- `context_server_command` and `context_server_configuration`: MCP
  context servers
- `suggest_docs_packages` and `index_docs`: documentation providers
  for the agent's docs lookup

Debugging (DAP):

- `get_dap_binary`, `dap_request_kind`, `dap_config_to_scenario`,
  `dap_locator_create_scenario`, `run_dap_locator`: describe and
  locate debug adapters and translate debug configurations

## Host interfaces (WIT worlds)

The `extension` world in `wit/since_v0.8.0/` is split into interfaces:

| Interface | What it provides |
| --- | --- |
| `common` | Shared types: ranges, code labels, settings, worktree information |
| `lsp` | Language server protocol types and completion/symbol labels |
| `dap` | Debug adapter protocol types and scenarios |
| `slash-command` | Agent slash command types |
| `context-server` | MCP context server types |
| `process` | Running declared commands |
| `http-client` | HTTP requests through the host |
| `github` | GitHub release and repository queries (for fetching server binaries) |
| `nodejs` | Locating the user's Node.js runtime and npm packages |
| `platform` | Operating system and architecture information |
| `settings` (Rust module) | Reading the user's settings for your extension |

## Building and installing an extension locally

```sh
cargo build --target wasm32-wasip1 --release
```

Then install it from the Extensions panel with **Install Dev
Extension** and pick the extension folder. Zaply validates the
manifest, builds the component if needed, and loads it. Dev extensions
can be rebuilt in place from the same panel while you iterate.

## Compatibility notes

- The API is at 0.8.0 and still evolving. Breaking changes are batched
  and listed in `crates/extension_api/PENDING_CHANGES.md` before they
  land.
- Extensions run on all three desktop platforms from the same Wasm
  component; anything platform-specific goes through the `platform`
  interface.
- There is no extension gallery run by Zaply. Distribution today means
  sharing the extension's repository (users install it as a dev
  extension) or the community registry described in
  `EXTENSION_PACKS.md`.
