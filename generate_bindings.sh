#!/bin/bash

# Reasons against rust-bindgen as a build-dependency:
#   - no real upside due to lack of architecture-specific definitions
#   - rust-bindgen carries many dependencies (including libclang)
#   - updating manually prevents bugs
${BINDGEN:-bindgen} --use-core --no-layout-tests gr/lib/gks/gks.h -o src/gks/bindings/gks.rs
${BINDGEN:-bindgen} --use-core --no-layout-tests --blocklist-type 'size_t' gr/lib/gks/gkscore.h -o src/gks/bindings/gkscore.rs
