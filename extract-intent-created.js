const { IntentSourceAbi } = require('@eco-foundation/routes-ts');

// Find IntentCreated event
const intentCreatedEvent = IntentSourceAbi.find(item => 
  item.type === 'event' && item.name === 'IntentCreated'
);

if (intentCreatedEvent) {
  console.log('IntentCreated Event Structure:');
  console.log(JSON.stringify(intentCreatedEvent, null, 2));
  
  console.log('\nInput Fields:');
  intentCreatedEvent.inputs.forEach((input, index) => {
    console.log(`${index}: ${input.name} (${input.type}) - indexed: ${input.indexed || false}`);
  });
} else {
  console.log('IntentCreated event not found');
}