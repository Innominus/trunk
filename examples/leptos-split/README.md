Trunk | Leptos Split
====================

An example application demonstrating Trunk's split-WASM support with modern Leptos.

It includes a wide mix of lazy routes and explicitly named lazy functions. Some routes own multiple
lazy functions, some share the same nested helper across routes, and `/inventory` creates a lazy
function from route data itself. This makes the emitted split manifest easier to inspect for shared
chunks, sibling split points, and nested chains.

Once you've installed this Trunk fork, run `trunk serve --open` from this example directory.
