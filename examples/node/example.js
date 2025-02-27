const { ZeroizedString } = require('../../pkg/node/wasm_zeroize.js');

// Create a secure string
const password = new ZeroizedString('my-secret-password');

// Use the secure string
const passwordValue = password.get_value();
console.log('Password value:', passwordValue);

// Explicitly clear it
password.zeroize();
console.log(
  'After zeroize, attempting to access should show empty or throw error'
);
try {
  const emptyValue = password.get_value();
  console.log('Value after zeroize:', emptyValue);
} catch (e) {
  console.log('Error after zeroize:', e.message);
}

console.log('Node.js example completed successfully!');
