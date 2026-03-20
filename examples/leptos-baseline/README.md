Trunk | Leptos Baseline
=======================

This example mirrors `examples/leptos-split`, but keeps the same route and interaction flow in a
single eagerly loaded bundle.

It is useful for comparing output sizes against `examples/leptos-split` without wasm-split
packaging or the split-specific relocation build flag.

Once you've installed Trunk, simply execute `trunk serve --open` from this example's directory to inspect the eager baseline locally.
