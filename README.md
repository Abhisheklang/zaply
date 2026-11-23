# Zaply - a fast, open source code editor

![Zaply app icon: a yellow lightning bolt on a dark rounded square, the symbol of Zaply's focus on speed](crates/zaply/resources/app-icon@2x.png)

Zaply is a fast, open source code editor built for speed: GPU-accelerated editing, a built-in AI agent panel, no account and no telemetry. 100% open source.

Download the latest Linux build from the [releases page](https://github.com/Abhisheklang/zaply/releases), or read on to see why Zaply exists.

## Why another code editor?

Most modern editors ask for something before you write a line of code: an account, a sign-in, a trial, or your usage data. Zaply asks for nothing. You download it, open a folder, and start editing. It is a full-featured code editor and a practical open source IDE alternative for people who want their tools fast, private, and theirs.

## Built for speed: GPU-accelerated editing

Zaply is a GPU-accelerated editor. Text, cursors, and interface elements are drawn by your graphics card instead of being painted pixel by pixel on the CPU, which is why scrolling through a very large file and typing with multiple cursors stay smooth.

What that means for you:

- **Instant startup** and fast file loading, even in large projects.
- **Responsive keystrokes** with latency low enough that the editor never feels like it is catching up with you.
- **Smooth scrolling and rendering** in big files, long lines, and split views.
- **Efficient use of every CPU core** for search, parsing, and language features, so background work does not block your typing.
- A syntax-aware editing engine that understands your code as a tree, powering precise highlighting, structural selections, and accurate auto-indent.

Zaply is written in Rust, chosen for the same reason: native performance without a runtime tax.

## A built-in AI code editor panel: bring your own API key

Zaply has a built-in AI agent panel. The agent can answer questions about your codebase, edit files, run terminal commands with your approval, and show you every change as a reviewable diff before it lands.

You stay in control of the AI, because there is no Zaply AI service:

- **Bring your own API key.** Connect the provider you already use: Anthropic, OpenAI, Google, OpenRouter, DeepSeek, Mistral, Amazon Bedrock, GitHub Copilot, or any OpenAI-compatible endpoint.
- **Or run local models.** Use Ollama or LM Studio and the agent works fully offline, with your code never leaving your machine.
- **Your keys stay yours.** API keys are stored on your computer, and requests go directly from Zaply to the provider you picked.
- **Inline edit predictions** are available from supported providers you configure, such as GitHub Copilot.

### External agents through the Agent Client Protocol (ACP)

Zaply also works with external coding agents through the Agent Client Protocol (ACP). ACP is an open protocol that lets a separate agent program connect to an editor and work inside it: the agent runs as its own process, while Zaply provides the conversation panel, file editing, tool permissions, and diff review. If an agent speaks ACP, you can add it in settings and use it in the same panel as the built-in providers, so you are never locked in to one AI vendor.

## No account, no sign-in, no Zaply server

There is nothing to sign up for. Zaply has:

- No user accounts and no activation.
- No trial timers, plans, or billing prompts.
- No Zaply cloud or hosted backend of any kind.

Features that depended on a hosted account in the editor Zaply is based on were removed outright, not rebranded: account sign-in, hosted billing and trials, hosted edit predictions, and hosted collaboration are simply not in the program.

## No telemetry, by design

Zaply does not phone home. The telemetry code was removed from the program entirely, not just switched off in settings:

- No usage metrics are collected or sent.
- No crash reports are uploaded anywhere.
- There are no analytics identifiers and no background update checks to a vendor server. New versions are published on the GitHub releases page, and you decide when to install them.

A local event log can be kept on your own machine to help you troubleshoot problems. It never leaves your computer. The only network connections Zaply makes are the ones you ask for: your chosen AI provider, your git remotes, and the language servers and tools you install.

## Features

**Editing**

- Multiple cursors and selections, with a fast, syntax-aware editing engine
- Language server support: autocompletion, go to definition, diagnostics, refactoring
- Built-in git integration: gutter changes, blame, staging, and branch tools
- Integrated terminal, task runner, and debugger
- Project-wide search and replace, outline panel, and symbol navigation
- Vim mode, plus keymaps modelled on the editors you already know
- A large theme system with light and dark themes, and full settings as editable files
- Per-project settings live in a `.zaply/` folder (settings, tasks, and debug configurations)

**AI**

- Built-in agent panel with threaded conversations and diff review
- Multi-provider support with your own API keys, or local models through Ollama and LM Studio
- External agents through the Agent Client Protocol (ACP)
- Tool calling with explicit permission prompts, so the agent only runs what you approve

**Privacy and ownership**

- No account, no telemetry, no hosted services
- Works offline for editing and for local-model AI
- 100% open source: read, audit, and modify every line

## Install Zaply on Linux

### Debian and Ubuntu (.deb package)

Download `zaply-linux-x86_64.deb` from the [releases page](https://github.com/Abhisheklang/zaply/releases), then install it:

```sh
sudo apt install ./zaply-linux-x86_64.deb
zaply
```

### Any Linux distribution (AppImage)

Download `zaply-linux-x86_64.AppImage` from the [releases page](https://github.com/Abhisheklang/zaply/releases), make it executable, and run it:

```sh
chmod +x zaply-linux-x86_64.AppImage
./zaply-linux-x86_64.AppImage
```

### Install script

The install script downloads the latest release and installs Zaply under `~/.local`, with no administrator rights needed:

```sh
curl -fsSL https://raw.githubusercontent.com/Abhisheklang/zaply/main/install.sh | sh
```

Or download `install.sh` from this repository and run it directly. Re-running it upgrades Zaply in place.

## Install Zaply on macOS

Download `Zaply-universal.dmg` from the [releases page](https://github.com/Abhisheklang/zaply/releases), open it, and drag Zaply into your Applications folder. The universal build runs on both Apple silicon and Intel Macs, macOS 12 or later.

### First launch on macOS

Zaply is not notarized by Apple, so the first time you open it, macOS may say "Zaply can't be opened because Apple cannot check it for malicious software." That is expected for an open source build distributed outside the App Store. To open it:

1. Right-click Zaply in Applications and choose Open, then confirm. If your macOS does not offer an Open button, open System Settings, go to Privacy & Security, and click Open Anyway next to the Zaply message.
2. If macOS still refuses, or says the app is damaged, run this in Terminal and then open Zaply normally:

```sh
xattr -dr com.apple.quarantine /Applications/Zaply.app
```

You only need to do this once.

## Install Zaply on Windows

Download `Zaply-x86_64.exe` from the [releases page](https://github.com/Abhisheklang/zaply/releases) and run the installer. Windows 10 or later.

### First run on Windows

The Zaply installer is not code-signed, so Windows Defender SmartScreen may stop it with "Windows protected your PC" and "Microsoft Defender SmartScreen prevented an unrecognized app from starting." That is expected for an open source build distributed outside the Microsoft Store. To continue:

1. Click **More info** in the SmartScreen dialog.
2. Click **Run anyway**.

The installer then runs normally. You only need to do this once.

## Build Zaply from source

To build the fast code editor from source on Linux:

```sh
git clone https://github.com/Abhisheklang/zaply.git
cd zaply
./script/linux        # installs the system packages needed to build
cargo build --release --bin zaply --bin cli
```

The editor binary is produced at `target/release/zaply`.

## Platforms

- **Linux**: available now, and the first supported platform for the v1.0 release.
- **Windows and macOS**: following from the same codebase in later releases. Zaply is not a Linux-only project; v1.0 is simply Linux first.

## Known limits in v1.0

Honesty matters more than a long feature list:

- Zaply has no extension store and does not load third-party extensions. Language support is built in instead: the grammars and language configuration for the bundled language set (including HTML, Java, GLSL, and Proto) are compiled into the app and work fully offline, with no install step.
- Updates are manual: download the newest build from the releases page. Zaply never checks a vendor server in the background.
- The server component for SSH remote development is not bundled yet.
- Hosted collaboration features were removed along with the account system they required.

## FAQ

**Is Zaply free?**
Yes. Zaply is free to download, use, and modify, and it is 100% open source under the GPL.

**Does Zaply need an account?**
No. There is no sign-in, no activation, and no Zaply server. The only accounts involved are with AI providers you choose to connect yourself.

**Does Zaply send telemetry?**
No. The telemetry sender was removed from the code, so there is nothing to opt out of. No usage data and no crash reports leave your machine.

**Which AI providers work with Zaply?**
Anthropic, OpenAI, Google, OpenRouter, DeepSeek, Mistral, Amazon Bedrock, GitHub Copilot, and any OpenAI-compatible endpoint, using your own API key. Local models work through Ollama and LM Studio. External agents that speak the Agent Client Protocol (ACP) can also be added.

**Can I use Zaply offline?**
Yes. Editing is fully offline, and with a local model through Ollama or LM Studio the AI agent is offline too. Only features that are networked by nature, like git push or an online AI provider, need a connection.

**Which platforms does Zaply run on?**
Linux today. Windows and macOS builds follow from the same codebase.

**How do I update Zaply?**
Download the newest package from the [releases page](https://github.com/Abhisheklang/zaply/releases) and install it over the old one, or re-run the install script.

## Contributing

Bug reports and pull requests are welcome on [GitHub Issues](https://github.com/Abhisheklang/zaply/issues). If you file a bug, the issue templates in this repository walk you through the details that help most.

## Licence

Zaply is free and open source software, licensed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). Some bundled libraries in this repository are under the Apache License 2.0 or the MIT License, as noted in their respective LICENSE files.

Attribution: Zaply is a fork of the Zed code editor, originally created by Zed Industries, Inc. The upstream copyright and licence notices are preserved in the LICENSE and NOTICE files in this repository, as the GPL requires. A list of the files changed in the fork is kept in [MODIFICATIONS.md](MODIFICATIONS.md).
