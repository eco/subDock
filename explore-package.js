try {
  const routes = require('@eco-foundation/routes-ts');
  console.log('Available exports:', Object.keys(routes));
  
  // Try to find IntentSource in different ways
  if (routes.IntentSource) {
    console.log('IntentSource found directly');
  } else if (routes.contracts && routes.contracts.IntentSource) {
    console.log('IntentSource found in contracts');
  } else {
    console.log('Full routes object:', JSON.stringify(routes, null, 2));
  }
} catch (error) {
  console.error('Error loading package:', error.message);
}