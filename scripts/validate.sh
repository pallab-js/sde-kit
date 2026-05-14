#!/usr/bin/env bash
set -euo pipefail

echo "==> Validating SDE-KIT build constraints..."

errors=0

# 1. Check for prohibited dependencies in Cargo workspace
PROHIBITED_CRATES="reqwest|hyper|tower|tokio-tungstenite|async-tungstenite"
for crate in crates/*/Cargo.toml apps/desktop/src-tauri/Cargo.toml; do
  if grep -qE "$PROHIBITED_CRATES" "$crate" 2>/dev/null; then
    echo "❌ Network/prohibited crate detected in $crate"
    errors=$((errors + 1))
  fi
done

# 2. Verify SQLite is bundled
if grep -q 'rusqlite' apps/desktop/src-tauri/Cargo.toml && \
   ! grep -q 'bundled' apps/desktop/src-tauri/Cargo.toml; then
  echo "❌ rusqlite must use 'bundled' feature for portability"
  errors=$((errors + 1))
fi

# 3. Check for prohibited references in frontend
if grep -qr 'firebase\|supabase\|auth0\|aws\.\|gcp\.\|azure\.' apps/desktop/src --include='*.svelte' --include='*.ts' 2>/dev/null; then
  echo "❌ Cloud/auth references found in frontend source"
  errors=$((errors + 1))
fi

# 4. Check binary size if release build exists
RELEASE_BIN="apps/desktop/src-tauri/target/release/sde-kit"
if [ -f "$RELEASE_BIN" ]; then
  size=$(stat -f%z "$RELEASE_BIN" 2>/dev/null || stat -c%s "$RELEASE_BIN" 2>/dev/null || echo 0)
  if [ "$size" -gt $((80 * 1024 * 1024)) ] 2>/dev/null; then
    echo "⚠️  Release binary is larger than 80MB ($((size / 1024 / 1024))MB)"
  fi
fi

# 5. Check Rust compiles
echo "→ Checking Rust compilation..."
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml 2>/dev/null || {
  echo "⚠️  Cargo check failed (may need dependencies installed)"
  errors=$((errors + 1))
}

# 6. Run graph crate tests
echo "→ Running graph crate tests..."
cargo test -p sde-kit-graph --quiet 2>/dev/null || {
  echo "⚠️  Graph crate tests failed"
  errors=$((errors + 1))
}

if [ "$errors" -gt 0 ]; then
  echo "❌ Validation failed: $errors error(s) found"
  exit 1
fi

echo "✅ All constraints validated."
