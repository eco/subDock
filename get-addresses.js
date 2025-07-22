const { EcoProtocolAddresses } = require('@eco-foundation/routes-ts');

console.log('Base Chain ID 8453 Contract Addresses:');
console.log('IntentSource:', EcoProtocolAddresses['8453'].IntentSource);
console.log('Inbox:', EcoProtocolAddresses['8453'].Inbox);
console.log('HyperProver:', EcoProtocolAddresses['8453'].HyperProver);
console.log('MetaProver:', EcoProtocolAddresses['8453'].MetaProver);