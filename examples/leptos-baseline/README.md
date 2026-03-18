Trunk | Leptos Baseline
=======================

This example mirrors `examples/leptos-split`, but keeps the same route and interaction flow in a
single eagerly loaded bundle.

It is useful for comparing output sizes against `examples/leptos-split` without split-WASM
packaging or the split-specific relocation build flag.

Run `trunk serve --open` from this example directory to inspect the eager baseline locally.
