mod pb;
mod schema;

use pb::intentsource::v1::{
    IntentEvents, IntentCreated, IntentFunded, IntentPartiallyFunded, 
    Withdrawal, Refund, IntentProofChallenged, Fulfillment, OrderFilled,
    TokenAmount, Call
};
use substreams::prelude::*;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[allow(unused_imports)]
use hex_literal::hex;

// IntentSource contract events - computed from ABI
const INTENT_CREATED_EVENT_SIG: [u8; 32] = hex!("d74850080f412f4e5145eb98178a1606b34e0b34878d7e2321614b83da2d1249");
const INTENT_FUNDED_EVENT_SIG: [u8; 32] = hex!("2da42efda5225344c30e729dc0eafc2e56292ac9b9b5c2b16e0e74c86ea5921d");
const INTENT_PARTIALLY_FUNDED_EVENT_SIG: [u8; 32] = hex!("97cf148f008486c490afd3b522e2398d5039247c7fffe81fcae2a8c6ee622103");
const WITHDRAWAL_EVENT_SIG: [u8; 32] = hex!("6653a45d3871e4110fa55dac0269f9f93a6d9078d402f7153594e50573d7f0cd");
const REFUND_EVENT_SIG: [u8; 32] = hex!("0ba6f12b978882904e7444c7a8fcadd2d9f692a6a97aa18e5fb44c3bbc580123");
const INTENT_PROOF_CHALLENGED_EVENT_SIG: [u8; 32] = hex!("69f2194063569059c6cc65d4599038f27aa9590bbb3f008178b6d20c453b9e82");

// Inbox contract events
const FULFILLMENT_EVENT_SIG: [u8; 32] = hex!("4a817ec64beb8020b3e400f30f3b458110d5765d7a9d1ace4e68754ed2d082de");
const ORDER_FILLED_EVENT_SIG: [u8; 32] = hex!("0555709e59fb225fcf12cc582a9e5f7fd8eea54c91f3dc500ab9d8c37c507770");

const INTENTSOURCE_CONTRACT_ADDRESS: [u8; 20] = hex!("2020ae689ED3e017450280CEA110d0ef6E640Da4");
const INBOX_CONTRACT_ADDRESS: [u8; 20] = hex!("04c816032A076dF65b411Bb3F31c8d569d411ee2");

#[substreams::handlers::map]
fn map_intent_events(blk: eth::Block) -> Result<IntentEvents, substreams::errors::Error> {
    let mut events = IntentEvents::default();

    for trx in blk.transaction_traces.iter() {
        for call in trx.calls.iter() {
            for log in call.logs.iter() {
                // Check if log is from our target contracts
                let is_intentsource = log.address == INTENTSOURCE_CONTRACT_ADDRESS;
                let is_inbox = log.address == INBOX_CONTRACT_ADDRESS;
                
                if !is_intentsource && !is_inbox {
                    continue;
                }

                if log.topics.is_empty() {
                    continue;
                }

                let topic = log.topics[0].as_slice();
                let block_number = blk.number;
                let tx_hash = format!("0x{}", hex::encode(&trx.hash));
                let timestamp = blk.timestamp.as_ref().unwrap().seconds as u64;

                match topic {
                    // IntentSource events
                    INTENT_CREATED_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_intent_created_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_created.push(event);
                        }
                    },
                    INTENT_FUNDED_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_intent_funded_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_funded.push(event);
                        }
                    },
                    INTENT_PARTIALLY_FUNDED_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_intent_partially_funded_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_partially_funded.push(event);
                        }
                    },
                    WITHDRAWAL_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_withdrawal_event(log, block_number, &tx_hash, timestamp) {
                            events.withdrawal.push(event);
                        }
                    },
                    REFUND_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_refund_event(log, block_number, &tx_hash, timestamp) {
                            events.refund.push(event);
                        }
                    },
                    INTENT_PROOF_CHALLENGED_EVENT_SIG if is_intentsource => {
                        if let Some(event) = decode_intent_proof_challenged_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_proof_challenged.push(event);
                        }
                    },
                    // Inbox events
                    FULFILLMENT_EVENT_SIG if is_inbox => {
                        if let Some(event) = decode_fulfillment_event(log, block_number, &tx_hash, timestamp) {
                            events.fulfillment.push(event);
                        }
                    },
                    ORDER_FILLED_EVENT_SIG if is_inbox => {
                        if let Some(event) = decode_order_filled_event(log, block_number, &tx_hash, timestamp) {
                            events.order_filled.push(event);
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: IntentEvents) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for event in events.intent_created {
        tables
            .create_row("intent_created", &event.intent_hash)
            .set("intent_hash", &event.intent_hash)
            .set("salt", &event.salt)
            .set("source_chain_id", event.source_chain_id)
            .set("destination_chain_id", event.destination_chain_id)
            .set("inbox_address", &event.inbox_address)
            .set("creator", &event.creator)
            .set("prover", &event.prover)
            .set("deadline", event.deadline)
            .set("native_value", event.native_value)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_funded {
        tables
            .create_row("intent_funded", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("funder", &event.funder)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_partially_funded {
        tables
            .create_row("intent_partially_funded", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("funder", &event.funder)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.withdrawal {
        tables
            .create_row("withdrawal", &format!("{}_{}", event.hash, event.block_number))
            .set("hash", &event.hash)
            .set("recipient", &event.recipient)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.refund {
        tables
            .create_row("refund", &format!("{}_{}", event.hash, event.block_number))
            .set("hash", &event.hash)
            .set("recipient", &event.recipient)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_proof_challenged {
        tables
            .create_row("intent_proof_challenged", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.fulfillment {
        tables
            .create_row("fulfillment", &format!("{}_{}", event.hash, event.block_number))
            .set("hash", &event.hash)
            .set("source_chain_id", event.source_chain_id)
            .set("prover", &event.prover)
            .set("claimant", &event.claimant)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.order_filled {
        tables
            .create_row("order_filled", &format!("{}_{}", event.order_id, event.block_number))
            .set("order_id", &event.order_id)
            .set("solver", &event.solver)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    Ok(tables.to_entity_changes())
}

// Event decoding functions
fn decode_intent_created_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentCreated> {
    if log.topics.len() < 4 {
        return None;
    }

    // Parse indexed fields from topics
    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let creator = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let prover = format!("0x{}", hex::encode(&log.topics[3][12..]));
    
    // For simplicity, parse basic fields from data
    // In production, you'd want to use proper ABI decoding
    let mut offset = 0;
    
    // Skip complex ABI decoding for now and extract basic fields
    let salt = if log.data.len() >= 32 {
        format!("0x{}", hex::encode(&log.data[offset..offset+32]))
    } else {
        "0x".to_string()
    };
    offset += 32;

    let source_chain_id = if log.data.len() >= offset + 32 {
        u64::from_be_bytes(log.data[offset+24..offset+32].try_into().unwrap_or([0; 8]))
    } else {
        0
    };
    offset += 32;

    let destination_chain_id = if log.data.len() >= offset + 32 {
        u64::from_be_bytes(log.data[offset+24..offset+32].try_into().unwrap_or([0; 8]))
    } else {
        0
    };
    offset += 32;

    let inbox_address = if log.data.len() >= offset + 32 {
        format!("0x{}", hex::encode(&log.data[offset+12..offset+32]))
    } else {
        "0x".to_string()
    };
    
    // For complex arrays like TokenAmount and Call, we'll skip detailed parsing
    // and leave them empty for now - proper ABI decoding would be needed
    
    Some(IntentCreated {
        intent_hash,
        salt,
        source_chain_id,
        destination_chain_id,
        inbox_address,
        route_tokens: vec![], // TODO: Implement proper ABI decoding
        calls: vec![], // TODO: Implement proper ABI decoding
        creator,
        prover,
        deadline: 0, // TODO: Extract from data
        native_value: 0, // TODO: Extract from data
        reward_tokens: vec![], // TODO: Implement proper ABI decoding
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_funded_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentFunded> {
    if log.data.len() < 64 {
        return None;
    }

    // Both fields are in data (not indexed)
    let intent_hash = format!("0x{}", hex::encode(&log.data[0..32]));
    let funder = format!("0x{}", hex::encode(&log.data[44..64])); // Skip padding

    Some(IntentFunded {
        intent_hash,
        funder,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_partially_funded_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentPartiallyFunded> {
    if log.data.len() < 64 {
        return None;
    }

    // Both fields are in data (not indexed)
    let intent_hash = format!("0x{}", hex::encode(&log.data[0..32]));
    let funder = format!("0x{}", hex::encode(&log.data[44..64])); // Skip padding

    Some(IntentPartiallyFunded {
        intent_hash,
        funder,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_withdrawal_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<Withdrawal> {
    if log.topics.len() < 2 || log.data.len() < 32 {
        return None;
    }

    // hash is in data, recipient is indexed
    let hash = format!("0x{}", hex::encode(&log.data[0..32]));
    let recipient = format!("0x{}", hex::encode(&log.topics[1][12..]));

    Some(Withdrawal {
        hash,
        recipient,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_refund_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<Refund> {
    if log.topics.len() < 2 || log.data.len() < 32 {
        return None;
    }

    // hash is in data, recipient is indexed
    let hash = format!("0x{}", hex::encode(&log.data[0..32]));
    let recipient = format!("0x{}", hex::encode(&log.topics[1][12..]));

    Some(Refund {
        hash,
        recipient,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_proof_challenged_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentProofChallenged> {
    if log.data.len() < 32 {
        return None;
    }

    // Only intentHash field in data (not indexed)
    let intent_hash = format!("0x{}", hex::encode(&log.data[0..32]));

    Some(IntentProofChallenged {
        intent_hash,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_fulfillment_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<Fulfillment> {
    if log.topics.len() < 4 || log.data.len() < 32 {
        return None;
    }

    // _hash, _sourceChainID, _prover are indexed; _claimant is in data
    let hash = format!("0x{}", hex::encode(&log.topics[1]));
    let source_chain_id = u64::from_be_bytes(log.topics[2][24..].try_into().ok()?);
    let prover = format!("0x{}", hex::encode(&log.topics[3][12..]));
    let claimant = format!("0x{}", hex::encode(&log.data[12..32])); // Skip padding

    Some(Fulfillment {
        hash,
        source_chain_id,
        prover,
        claimant,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_order_filled_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<OrderFilled> {
    if log.data.len() < 64 {
        return None;
    }

    // Both _orderId and _solver are in data (not indexed)
    let order_id = format!("0x{}", hex::encode(&log.data[0..32]));
    let solver = format!("0x{}", hex::encode(&log.data[44..64])); // Skip padding

    Some(OrderFilled {
        order_id,
        solver,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}