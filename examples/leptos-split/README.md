Trunk | Leptos Split
====================

An example application demonstrating Trunk's wasm-split support with modern Leptos.

It includes a wide mix of lazy routes and explicitly named lazy functions. Some routes own multiple
lazy functions, some share the same nested helper across routes, and `/inventory` creates a lazy
function from route data itself. This makes the emitted split manifest easier to inspect for shared
chunks, sibling split points, and nested chains.

Once you've installed Trunk, simply execute `trunk serve --open` from this example's directory, and you should see the web application rendered in your browser.
