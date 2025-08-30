# XOR Crusher LV2 Plugin

This is a simple LV2 audio plugin written in Rust. It takes two audio inputs, quantizes them to a user-defined bit depth, performs a bitwise XOR operation on them, and sends the result to a single audio output.

## Prerequisites

- Rust and Cargo (https://rustup.rs/)
- A Linux-based operating system
- `zip` and `make` command-line utilities (usually pre-installed)

## How to Build and Use

1.  **Build the Plugin and Create the LV2 Bundle**

    Open your terminal in this directory and run:
    ```bash
    make
    ```
    This will compile the Rust code and create a `xor_crusher.lv2` directory inside the `target` folder.

2.  **Install the Plugin**

    To make the plugin available to audio applications (like Ardour, REAPER, or Carla), install it to your local LV2 directory:
    ```bash
    make install
    ```

3.  **Create a Distributable Zip File**

    To package the plugin for others to use, run:
    ```bash
    make zip
    ```
    This will create a `xor_crusher.lv2.zip` file in the project's root directory.

4.  **Clean Up**

    To remove all build files:
    ```bash
    make clean
    ```
