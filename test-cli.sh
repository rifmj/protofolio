#!/bin/bash
# Test script for protofolio-cli

set -e

echo "=== Testing protofolio-cli ==="
echo ""

# Generate AsyncAPI spec from Rust example
echo "1. Generating AsyncAPI spec from Rust example..."
cargo run --example basic --package protofolio 2>/dev/null | \
  grep -A 1000 "Generated AsyncAPI specification (JSON)" | \
  grep -v "Generated AsyncAPI specification (JSON)" | \
  grep -v "Generated AsyncAPI specification (YAML)" | \
  grep -B 1000 "Generated AsyncAPI specification (YAML)" | \
  head -n -1 > /tmp/test-asyncapi.json

if [ ! -s /tmp/test-asyncapi.json ]; then
  echo "Failed to extract JSON spec"
  exit 1
fi

echo "✓ Spec generated: $(wc -l < /tmp/test-asyncapi.json) lines"
echo ""

# Build the CLI
echo "2. Building protofolio-cli..."
cargo build --release --package protofolio-cli
echo "✓ CLI built successfully"
echo ""

# Test the CLI
echo "3. Testing TypeScript generation..."
OUTPUT_DIR="/tmp/protofolio-types"
rm -rf "$OUTPUT_DIR"

./target/release/protofolio generate \
  --spec /tmp/test-asyncapi.json \
  --output "$OUTPUT_DIR"

if [ -d "$OUTPUT_DIR" ] && [ "$(ls -A $OUTPUT_DIR)" ]; then
  echo ""
  echo "✓ TypeScript types generated successfully!"
  echo "  Output directory: $OUTPUT_DIR"
  echo ""
  echo "Generated files:"
  ls -lh "$OUTPUT_DIR"
  echo ""
  echo "Sample generated type:"
  head -20 "$OUTPUT_DIR"/*.ts 2>/dev/null || echo "No .ts files found"
else
  echo "✗ Failed to generate types"
  exit 1
fi

echo ""
echo "=== Test completed successfully ==="

