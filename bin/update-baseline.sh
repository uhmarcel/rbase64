#!/bin/bash

# Run from root directory
(echo '# Profiling Report' && echo '```diff' && cargo bench --bench benchmarks -- --baseline main && echo '```') | ./bin/diff.js > ./benches/baseline.md