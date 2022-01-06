# Prisma Engines for Raspberry Pi 4

## Original Source Code

1. https://github.com/prisma/prisma-engines

## Checkout

1. https://www.prisma.io/docs/concepts/components/prisma-engines
2. https://github.com/prisma/prisma-engines
   1. Version of Raspbian
      * No LSB modules are available.
      * Distributor ID: Raspbian
      * Description: Raspbian GNU/Linux 10 (buster)
      * Release: 10
      * Codename: buster

## Raspberry pi 4 additional prerequisites

1. Install rustup: https://rustup.rs/
   > sudo apt install direnv
2. Put following in ending lines of ~/.bashrc
   > eval "$(direnv hook bash)"
3. durenv allow on repository root
   > sudo apt install protobuf-compiler
4. Building with ld causes an error. so we need to change linker from ld to gcc
   > sudo apt install gcc-arm-linux-gnueabihf
5. Write ~/.cargo/config following
   shell```
      [target.armv7-unknown-linux-gnueabihf]
      linker = "arm-linux-gnueabihf-gcc"
   ```shell

6. execute cargo build on the repository root
   > cargo build
7. or cargo build --release for release
   * Finished release in 158m 53s

## Repository Links

1. Prisma Engine for Raspberry Pi 4: https://github.com/siderdax/prisma4rpi4
2. Example of Next.js + Prisma Engine for Raspberry Pi 4: 
