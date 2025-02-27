# wasm-zeroize

A WebAssembly module for secure sensitive data handling with automatic memory zeroization.

## Features

- Secure containers for sensitive data
- Automatic memory zeroization when containers are dropped
- WebAssembly implementation for use in web applications and Node.js
- Zero dependencies in browser environments

## Installation

```
npm install wasm-zeroize
# or
yarn add wasm-zeroize
# or
pnpm add wasm-zeroize
```

## Usage

### Node.js

```javascript
import { ZeroizedString } from 'wasm-zeroize';

// Create a secure string
const password = new ZeroizedString('my-secret-password');

// Use the secure string
const passwordValue = password.get_value();
console.log(passwordValue); // "my-secret-password"

// Memory is automatically zeroized when the object is garbage collected
// Or explicitly clear it:
password.zeroize();
```

### Browser

```html
<script type="module">
  // Import the module
  import * as wasmModule from 'wasm-zeroize';

  async function run() {
    // Initialize the WebAssembly module
    await wasmModule.default();

    // Now we can use the module
    const { ZeroizedString } = wasmModule;

    // Create a secure string
    const password = new ZeroizedString('my-secret-password');

    // Use the secure string
    const passwordValue = password.get_value();
    console.log(passwordValue); // "my-secret-password"

    // Memory is automatically zeroized when the object is garbage collected
    // Or explicitly clear it:
    password.zeroize();
  }

  run();
</script>
```

## Building from Source

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Run the build script
chmod +x build.sh
./build.sh
```

## Examples

Check out the examples directory for working examples:

- `examples/node/example.js` - Node.js example
- `examples/browser/index.html` - Browser example

## Security Considerations

- This module uses the `zeroize` crate to ensure that sensitive data is properly cleared from memory when no longer needed.
- Memory zeroization helps prevent sensitive information from being leaked through memory dumps or other side-channel attacks.
- While this library provides tools for secure memory handling, it cannot protect against all types of attacks. Always follow security best practices.

## Compatibility

- Works in all modern browsers that support WebAssembly
- Compatible with Node.js environments
- Requires the `zeroize` and `console_error_panic_hook` crates

## License

[MIT License](LICENSE)
