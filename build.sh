#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <image-location> <new-image-name>"
    exit 1
fi


IMAGE_LOCATION="$1"
NEW_IMAGE_NAME="$2"

cd rpeg || { echo "Failed to enter directory /rpeg. Exiting..."; exit 1; }

cargo build --release || { echo "Cargo build failed. Exiting..."; exit 1; }


./target/release/rpeg -c "$IMAGE_LOCATION" >> compressed.txt || { echo "Compression failed. Exiting..."; exit 1; }

./target/release/rpeg -d compressed.txt >> "${NEW_IMAGE_NAME}.ppm" || { echo "Decompression failed. Exiting..."; exit 1; }

rm compressed.txt

mv "${NEW_IMAGE_NAME}.ppm" "$(pwd -P)/../"

echo "Process completed. The decompressed image is located at $(pwd -P)/../${NEW_IMAGE_NAME}.ppm"
