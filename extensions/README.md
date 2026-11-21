# Zaply Extensions

This directory contains extensions for Zaply that are largely maintained by the Zaply team. They currently live in the Zaply repository for ease of maintenance.

If you are looking for the Zaply extension registry, see the [`zed-industries/extensions`](https://github.com/zed-industries/extensions) repo.

## Structure

Currently, Zaply includes support for a number of languages without requiring installing an extension. Their grammars, queries, and configs can be found under [`crates/grammars/src`](https://github.com/Abhisheklang/zaply/tree/main/crates/grammars/src), and their language server adapters under [`crates/languages/src`](https://github.com/Abhisheklang/zaply/tree/main/crates/languages/src).

Support for all other languages is done via extensions. This directory ([extensions/](https://github.com/Abhisheklang/zaply/tree/main/extensions/)) contains some of the officially maintained extensions. These extensions use the same [zaply_extension_api](https://docs.rs/zaply_extension_api/latest/zaply_extension_api/) available to all [Zaply Extensions](https://github.com/Abhisheklang/zaply) for providing [language servers](https://github.com/Abhisheklang/zaply), [tree-sitter grammars](https://github.com/Abhisheklang/zaply) and [tree-sitter queries](https://github.com/Abhisheklang/zaply).

You can find the other officially maintained extensions in the [zaply-extensions organization](https://github.com/zaply-extensions).

## Dev Extensions

See the docs for [Developing an Extension Locally](https://github.com/Abhisheklang/zaply) for how to work with one of these extensions.
