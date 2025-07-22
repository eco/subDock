const { IntentSourceAbi, InboxAbi } = require('@eco-foundation/routes-ts');

console.log('=== INTENTSOURCE CONTRACT EVENTS ===\n');

const intentSourceEvents = IntentSourceAbi.filter(item => item.type === 'event');

intentSourceEvents.forEach(event => {
  console.log(`EVENT: ${event.name}`);
  console.log('Fields:');
  event.inputs.forEach((input, index) => {
    const indexedStr = input.indexed ? ' (indexed)' : '';
    console.log(`  ${index}: ${input.name} (${input.type})${indexedStr}`);
  });
  console.log('---');
});

console.log('\n=== INBOX CONTRACT EVENTS ===\n');

const inboxEvents = InboxAbi.filter(item => item.type === 'event');

inboxEvents.forEach(event => {
  console.log(`EVENT: ${event.name}`);
  console.log('Fields:');
  event.inputs.forEach((input, index) => {
    const indexedStr = input.indexed ? ' (indexed)' : '';
    console.log(`  ${index}: ${input.name} (${input.type})${indexedStr}`);
  });
  console.log('---');
});

console.log('\n=== COMPLETE EVENT STRUCTURES ===\n');

console.log('IntentSource Events:');
intentSourceEvents.forEach(event => {
  console.log(`${event.name}:`);
  console.log(JSON.stringify(event, null, 2));
  console.log('\n');
});

console.log('Inbox Events:');
inboxEvents.forEach(event => {
  console.log(`${event.name}:`);
  console.log(JSON.stringify(event, null, 2));
  console.log('\n');
});