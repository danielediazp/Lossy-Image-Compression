

# Simple JPEG

## Overview
This project presents a Rust implementation of a lossy image compression algorithm, specifically designed to work with JPEG format. It leverages the efficiency and safety of Rust to provide a high-performance solution for image compression needs.

## Features
- Lossy JPEG compression: Efficiently reduces the file size of images while maintaining a balance between quality and compression.
- PPM Image Support: Currently supports `.ppm` format images, making it suitable for a variety of applications and use cases.

### Build With
![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

## Getting Started

### Prerequisites
Before running this program, ensure you have Rust installed on your system. Rust's toolchain, including `cargo`, should be properly configured to compile and run Rust projects. If you're new to Rust, follow the official [installation guide](https://www.rust-lang.org/tools/install) to set up your environment.


### Installation 
1. Clone the repository to your local machine:
     ```sh
      git clone https://github.com/danielediazp/Lossy-Image-Compression.git
    ```

### How to Run
To compress and decompress your images, follow these steps:

* Ensure Execution Permissions for build.sh:

Before running build.sh, you must ensure it has execution permissions. To do this, navigate to the directory containing build.sh and run the following command:
```sh
      chmod +x build.sh
```

* Running the Script:

With execution permissions set, you can now run the script by passing the location of the image to compress and the desired name for the new image as arguments:
```sh
    ./build.sh /path/to/image.ppm new_image_name
   ```

Replace /path/to/image.ppm with the path to your .ppm format image and new_image_name with the desired name for the output image.

The script will generate the compressed image in the current directory from where you call the build.sh script. The output format will also be a .ppm image.

