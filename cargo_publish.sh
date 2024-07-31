#!/bin/bash

set -e

# Function to publish a crate
publish_crate() {
    local crate_path=$1
    echo "Publishing crate in $crate_path..."
    cd "$crate_path"
    cargo publish
    cd - > /dev/null
}

# Publish crates in the crates folder
for crate in crates/*; do
    if [ -d "$crate" ]; then
        publish_crate "$crate"
    fi
done

# Publish the main crate
echo "Publishing main crate..."
publish_crate "."

echo "All crates published successfully!"
