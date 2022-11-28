#!/bin/bash

# Run from root directory
cargo bench --bench benchmarks -- --save-baseline head

(echo '# Profiling Report' && echo '```diff' && cargo bench --bench benchmarks -- --load-baseline head --baseline main && echo '```') | ./scripts/diff.js > ./benches/baseline.md