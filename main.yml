name: build-aarch64

on:
  push:
    branches: ["main"]
  workflow_dispatch:

jobs:
  build-aarch64:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-unknown-linux-musl
      # Installs Rust + the specified target via rustup. [web:266]

      - name: Install cross
        run: cargo install cross --locked

      - name: Build (release, aarch64 musl)
        run: cross build --release --target aarch64-unknown-linux-musl
      # cross build with aarch64-unknown-linux-musl is a standard pattern. [web:265]

      - name: Prepare artifact
        run: |
          mkdir -p dist
          cp target/aarch64-unknown-linux-musl/release/digital-closet dist/digital-closet-aarch64
          chmod +x dist/digital-closet-aarch64

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: digital-closet-aarch64
          path: dist/digital-closet-aarch64
