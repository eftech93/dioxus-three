#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "========================================="
echo "Publishing dioxus-three to crates.io"
echo "========================================="
echo ""

echo "🔍 Formatting code..."
cargo fmt

echo "🔍 Running clippy..."
cargo clippy

echo "🧪 Running tests..."
cargo test

echo "📦 Verifying package..."
cargo publish --dry-run

echo ""
echo "========================================"
echo "🚀 Ready to publish!"
echo "========================================"
echo ""
echo "Author: Esteban Puello <eftech93@gmail.com>"
echo "Repository: https://github.com/eftech93/dioxus-three"
echo "Crate: dioxus-three"
echo ""
read -p "Continue with publish? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publish cancelled"
    exit 1
fi

echo "📦 Publishing dioxus-three..."
cargo publish

echo ""
echo "✅ dioxus-three published successfully!"
echo ""
echo "View at: https://crates.io/crates/dioxus-three"
echo "Docs at: https://docs.rs/dioxus-three"
