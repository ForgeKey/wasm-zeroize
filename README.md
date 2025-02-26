# wasm-zeroize

A WebAssembly module for secure sensitive data handling with automatic memory zeroization.

## Features

- Secure containers for sensitive data
- Automatic memory zeroization when containers are dropped
- WebAssembly implementation for use in web applications
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

### Basic Example

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

## Building from Source

To build the WebAssembly module:

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the package
wasm-pack build --target web
```

This will generate the necessary JavaScript and TypeScript bindings in the `pkg` directory.

## Security Considerations

- This module uses the `zeroize` crate to ensure that sensitive data is properly cleared from memory when no longer needed.
- Memory zeroization helps prevent sensitive information from being leaked through memory dumps or other side-channel attacks.
- While this library provides tools for secure memory handling, it cannot protect against all types of attacks. Always follow security best practices.

## Browser Compatibility

- Works in all modern browsers that support WebAssembly
- Requires the `zeroize` and `console_error_panic_hook` crates

## License

[MIT License](LICENSE)
