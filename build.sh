#!/usr/bin/env sh
# scripts/run-linux.sh
# Runs the correct prebuilt Linux binary (aarch64 or x86_64) from ./dist

set -eu

APP="digital-closet"
ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
DIST="$ROOT/dist"
ARCH="$(uname -m)"  # e.g. x86_64, aarch64 [web:558]

case "$ARCH" in
  x86_64|amd64)
    BIN="$DIST/$APP-x86_64"
    ;;
  aarch64|arm64)
    BIN="$DIST/$APP-aarch64"
    ;;
  *)
    echo "Unsupported architecture: $ARCH"
    echo "Expected x86_64/amd64 or aarch64/arm64."
    exit 1
    ;;
esac

if [ ! -f "$BIN" ]; then
  echo "Binary not found: $BIN"
  echo
  echo "Fix:"
  echo "1) Build/download binaries and place them in ./dist"
  echo "2) Ensure names match:"
  echo "   dist/$APP-x86_64"
  echo "   dist/$APP-aarch64"
  exit 1
fi

chmod +x "$BIN" || true  # make executable when possible [web:557]
exec "$BIN" "$@"
