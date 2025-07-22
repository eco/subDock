const { IntentSourceAbi, InboxAbi } = require('@eco-foundation/routes-ts');
const { keccak256 } = require('js-sha3');

// Extract ABIs and compute event signatures
console.log('IntentSource ABI:');
console.log(JSON.stringify(IntentSourceAbi, null, 2));

console.log('\n\nInbox ABI:');
console.log(JSON.stringify(InboxAbi, null, 2));

// Extract event signatures from IntentSource
const intentEvents = IntentSourceAbi.filter(item => item.type === 'event');
console.log('\nIntentSource Event Signatures:');

intentEvents.forEach(event => {
  const signature = `${event.name}(${event.inputs.map(input => input.type).join(',')})`;
  const hash = keccak256(signature);
  console.log(`${event.name}: ${signature} -> 0x${hash}`);
});

// Extract event signatures from Inbox
const inboxEvents = InboxAbi.filter(item => item.type === 'event');
console.log('\nInbox Event Signatures:');

inboxEvents.forEach(event => {
  const signature = `${event.name}(${event.inputs.map(input => input.type).join(',')})`;
  const hash = keccak256(signature);
  console.log(`${event.name}: ${signature} -> 0x${hash}`);
});

// Log keccak256 hashes for Solidity events
console.log('\nKeccak256 Event Hashes (for Rust):');
console.log('// IntentSource Events');
intentEvents.forEach(event => {
  const signature = `${event.name}(${event.inputs.map(input => input.type).join(',')})`;
  const hash = keccak256(signature);
  console.log(`const ${event.name.toUpperCase().replace(/-/g, '_')}_EVENT_SIG: [u8; 32] = hex!("${hash}");`);
});

console.log('\n// Inbox Events');
inboxEvents.forEach(event => {
  const signature = `${event.name}(${event.inputs.map(input => input.type).join(',')})`;
  const hash = keccak256(signature);
  console.log(`const ${event.name.toUpperCase().replace(/-/g, '_')}_EVENT_SIG: [u8; 32] = hex!("${hash}");`);
});