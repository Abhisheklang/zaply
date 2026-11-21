# Contributing to Zaply

Thank you for helping us make Zaply better!

## Before you start

- **Keep no more than three PRs open at a time.**
  This limit helps us keep up with reviews and ensures all contributors get a fair share of our reviewing capacity.
  Start with one and see it through; we're lucky to get a lot of contributions, and the pattern we've seen is that landing your first PR dramatically improves the odds for every PR after it.
  A stack of open PRs on the other hand tends to go stale as `main` changes.
- **Discuss features with us before you start writing code for them**.
  If there isn't a GitHub issue with staff confirmation that we want it, start a [GitHub discussion](https://github.com/Abhisheklang/zaply/discussions), not a PR or a new issue.
  This especially applies to changes to the Zaply Extension API.
  - Before proposing a larger feature, cover the context, integration points, and design decisions in your proposal.

All activity in Zaply forums is subject to our [Code of Conduct](https://github.com/Abhisheklang/zaply).

### Things we will (probably) not merge

There are few hard-and-fast rules, but we typically don't merge:

- **Changes that can be provided by an extension**, such as new languages or themes. See the [extension development docs](https://github.com/Abhisheklang/zaply).
- **Extension API changes without prior discussion** involving Zaply staff.
- **New file icons.** Our default icons are hand-designed to fit together; please don't submit off-the-shelf SVGs.
- **Features whose complexity outweighs their benefit**, in our judgement, for the number of people who would use them.
- **Giant refactorings.**
- **Non-trivial changes without tests.**
- **Style-only code changes that don't alter app logic.** Reducing allocations, removing `.unwrap()`s, and fixing typos are welcome; making code "more readable" alone may not be.
- **LLM-generated work the author doesn't understand.**

### We love PRs that...

- **Fix or extend the docs**: browse [docs issues](https://github.com/Abhisheklang/zaply/issues?q=is%3Aissue%20state%3Aopen%20type%3ADocs).
- **Close issues curated for the community:** [good first issues](https://github.com/Abhisheklang/zaply/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22.contrib%2Fgood%20first%20issue%22), [good non-first issues](https://github.com/Abhisheklang/zaply/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22.contrib%2Fgood%20non-first%20issue%22).
- **Fix bugs**: start with [triaged bugs with confirmed reproduction steps](https://github.com/Abhisheklang/zaply/issues?q=is%3Aissue%20state%3Aopen%20type%3ABug%20label%3Astate%3Areproducible), or browse [area labels](https://github.com/Abhisheklang/zaply/labels?q=area%3A*) for parts of Zaply you care about.
  Select a label, then add `type:Bug` to the search.
- **Make existing features work for more people** through small enhancements, such as support for more platforms or modes.
- **Add small features**, like keybindings or actions you miss from other editors.
- **Join a Community Program** like [Let's Git Together](https://github.com/Abhisheklang/zaply/issues/41541) or [The Guild](https://github.com/Abhisheklang/zaply).
- **Build features we've explicitly invited contributions for**, listed on the [issue tracker](https://github.com/Abhisheklang/zaply).

## AI Policy

We welcome the use of LLMs for coding, but we hold a high bar for all contributions, and **we expect a human in the loop who genuinely understands the work an LLM produces** on their behalf.

For that reason, we **don't accept contributions from autonomous agents**. Pull requests that appear to violate this may be closed without notice.

**Don't rely on LLMs to write the whole thing for you when communicating with the maintainers** (meaning replies to comments, PR descriptions, and alike). The readers are humans, and we'd like to hear from you, not from a model (we have models at home).

If you're a non-native English speaker using an LLM to thoroughly edit or translate your messages to the maintainers, we'd encourage you to **put the machine translation in a quote block and include the original text in your native language after it**.

If you think it's helpful/necessary to **share context from a chat with an LLM**, please put the **relevant part of it** in a quote block (e.g., using `>`), **disclose it as AI-generated**, and add your own commentary explaining **why it's relevant and what you take from it**.

This policy was adapted from [ripgrep's AI policy](https://github.com/BurntSushi/ripgrep/blob/f0cec341ab95c25c691ad3d5754d4bd9eedde21f/AI_POLICY.md).

## Sending changes

**Build and run Zaply locally, then manually test your changes before opening a PR.** You need a stable Rust toolchain; on Linux, `script/linux` installs the system dependencies.

When preparing your PR:

- **Review your own work**, including any AI-assisted code. Follow the [pull request template](./.github/pull_request_template.md?plain=1) for testing details and the self-review checklist.
- **For visual changes, attach screenshots or a video** and work through the UI/UX checklist below. For non-visual improvements and changes, include benchmarks or other artifacts produced when testing.
- **Keep each PR about one thing**, leaving unrelated features and refactoring for another PR.
- Contributions are licensed under the same terms as the project (see the LICENSE files at the repository root).

**Opening a PR does not guarantee a merge.** We may decline a change that doesn't fit Zaply's direction or quality standards, even after you've put work into it. Confirming interest early and following this guide give your PR the best chance.

**Pinging maintainers by username or emailing them does not raise your PR's priority**; it takes time away from review.

## Resources

### UI/UX checklist

For UI changes, use the UI/UX checklist below.

### Bird's-eye view of Zaply

For a tour of the main crates, see the [codebase overview](https://github.com/Abhisheklang/zaply) in the development docs.

### Packaging Zaply

Check our [notes for packaging Zaply](https://github.com/Abhisheklang/zaply).
