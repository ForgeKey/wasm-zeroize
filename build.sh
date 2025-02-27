#!/bin/bash
set -e

echo "Building wasm-zeroize..."

# Clean previous builds
rm -rf pkg-temp pkg-web pkg-node

# Build for web
echo "Building for web..."
wasm-pack build --target web --out-dir pkg-web --release

# Build for Node.js
echo "Building for Node.js..."
wasm-pack build --target nodejs --out-dir pkg-node --release

# Create the final package directory
mkdir -p pkg

# Copy package.json template and modify it
cp package.json.template pkg/package.json

# Copy README.md and LICENSE
cp README.md pkg/
cp LICENSE pkg/

# Copy web version
mkdir -p pkg/web
cp pkg-web/wasm_zeroize_bg.wasm pkg/web/
cp pkg-web/wasm_zeroize.js pkg/web/
cp pkg-web/wasm_zeroize.d.ts pkg/web/

# Copy Node.js version
mkdir -p pkg/node
cp pkg-node/wasm_zeroize_bg.wasm pkg/node/
cp pkg-node/wasm_zeroize.js pkg/node/
cp pkg-node/wasm_zeroize.d.ts pkg/node/

# Create index.js for environment detection
cat > pkg/index.js << 'EOL'
// Environment detection for browser vs Node.js
function isNode() {
  return (
    typeof process !== 'undefined' &&
    process.versions != null &&
    process.versions.node != null &&
    typeof window === 'undefined'
  );
}

if (isNode()) {
  // Node.js environment
  module.exports = require('./node/wasm_zeroize.js');
} else {
  // Browser environment
  module.exports = require('./web/wasm_zeroize.js');
}
EOL

echo "Build completed successfully!"
echo ""
echo "To test:"
echo "  node examples/node/example.js"
echo ""
echo "To test in browser (requires a web server due to CORS):"
echo "  npx http-server -p 8080"
echo "  Then open http://localhost:8080/examples/browser/index.html in your browser"
echo ""
echo "To publish to npm:"
echo "  cd pkg && npm publish" 