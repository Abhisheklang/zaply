# Zaply benchmarks

Measured with `script/zaply-benchmarks`, which launches a built Zaply
application tree on a virtual X display (Xvfb) with a software Vulkan
driver (SwiftShader), a scratch user profile, and a generated project
of 300 small files, one 20 MB log file, and one file containing a
unique search token. Each number is the best of three runs where the
script takes three.

These numbers describe one specific machine: a shared virtual machine
with 1.5 CPUs and no GPU, rendering through software Vulkan. They are
useful for before/after comparisons on the same machine, and say
nothing about performance on real hardware or relative to other
editors. No other editor was measured, and no comparison is claimed.

## Results

| Measurement | Zaply 1.0 | Zaply 3.0 (this build) |
| --- | --- | --- |
| Startup, launch to first window | 4.90 s | 0.78 s |
| Startup, launch to settled frame | 7.01 s | 6.40 s |
| Open a 20 MB file to settled frame | 17.58 s | 6.06 s |
| Project search for a unique token to settled frame | 15.00 s | 2.69 s |

The 3.0 column was measured on the 3.0 development build on
2026-11-21, on this same machine with this same script. Treat the
comparison as indicative, not exact: this is a shared virtual machine,
load from other tenants varies between runs, and the two columns were
measured hours apart. The numbers were not produced by an isolated
A/B test of the scan-exclusions change described below, so no part of
the difference is attributed to any single change; they are reported
because they are what the shipped script measured for the two builds.

"Settled frame" means the rendered output stopped changing for one
second. On a software renderer these times are dominated by first-time
shader and font work that a real GPU does much faster.

## What changed between the two builds

The performance pass in the 3.0 development cycle was deliberately
conservative, because the release profile was already tuned (thin LTO,
single codegen unit for most crates). The change with a real effect on
everyday speed is in the defaults: `file_scan_exclusions` now covers
dependency and build-output folders (node_modules, target, virtualenvs,
Python caches, dist, build, and framework cache folders), so project
search, the file finder, and indexing skip them unless the user removes
an entry. On projects with large dependency folders this removes most
of the files a scan would otherwise walk.

## Performance modes

Two command-palette actions apply named settings profiles. Both write
ordinary settings, which you can inspect and revert in your settings
file.

**Enable Low-End Hardware Mode** reduces continuous rendering work:
cursor blinking off, hover popovers off, minimap hidden.

**Enable Large Monorepo Mode** tunes a very large repository: inline
git blame off, git gutter updates debounced to 1000 ms, and the file
scan exclusions extended (keeping any you already had) with the
folders that dominate monorepos.

## Reproduce

```sh
script/zaply-benchmarks --app-dir path/to/zaply.app --output results.md
```

Requires Xvfb, ffmpeg, xdotool, and dbus-daemon, plus a SwiftShader
Vulkan driver for machines without a GPU.
