mod pb;
mod schema;

use pb::intentsource::v1::{
    IntentEvents, IntentCreated, IntentFunded, IntentPartiallyFunded, 
    Withdrawal, Refund, IntentProofChallenged
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

const CONTRACT_ADDRESS: [u8; 20] = hex!("2020ae689ED3e017450280CEA110d0ef6E640Da4");

#[substreams::handlers::map]
fn map_intent_events(blk: eth::Block) -> Result<IntentEvents, substreams::errors::Error> {
    let mut events = IntentEvents::default();

    for trx in blk.transaction_traces.iter() {
        for call in trx.calls.iter() {
            for log in call.logs.iter() {
                // Check if log is from our target contract
                if log.address != CONTRACT_ADDRESS {
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
                    INTENT_CREATED_EVENT_SIG => {
                        if let Some(event) = decode_intent_created_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_created.push(event);
                        }
                    },
                    INTENT_FUNDED_EVENT_SIG => {
                        if let Some(event) = decode_intent_funded_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_funded.push(event);
                        }
                    },
                    INTENT_PARTIALLY_FUNDED_EVENT_SIG => {
                        if let Some(event) = decode_intent_partially_funded_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_partially_funded.push(event);
                        }
                    },
                    WITHDRAWAL_EVENT_SIG => {
                        if let Some(event) = decode_withdrawal_event(log, block_number, &tx_hash, timestamp) {
                            events.withdrawal.push(event);
                        }
                    },
                    REFUND_EVENT_SIG => {
                        if let Some(event) = decode_refund_event(log, block_number, &tx_hash, timestamp) {
                            events.refund.push(event);
                        }
                    },
                    INTENT_PROOF_CHALLENGED_EVENT_SIG => {
                        if let Some(event) = decode_intent_proof_challenged_event(log, block_number, &tx_hash, timestamp) {
                            events.intent_proof_challenged.push(event);
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
            .set("creator", &event.creator)
            .set("source_chain_id", event.source_chain_id)
            .set("destination_chain_id", event.destination_chain_id)
            .set("reward_token", &event.reward_token)
            .set("reward_amount", &event.reward_amount)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_funded {
        tables
            .create_row("intent_funded", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("funder", &event.funder)
            .set("reward_token", &event.reward_token)
            .set("reward_amount", &event.reward_amount)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_partially_funded {
        tables
            .create_row("intent_partially_funded", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("funder", &event.funder)
            .set("reward_token", &event.reward_token)
            .set("reward_amount", &event.reward_amount)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.withdrawal {
        tables
            .create_row("withdrawal", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("claimant", &event.claimant)
            .set("reward_token", &event.reward_token)
            .set("reward_amount", &event.reward_amount)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.refund {
        tables
            .create_row("refund", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("recipient", &event.recipient)
            .set("reward_token", &event.reward_token)
            .set("reward_amount", &event.reward_amount)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    for event in events.intent_proof_challenged {
        tables
            .create_row("intent_proof_challenged", &format!("{}_{}", event.intent_hash, event.block_number))
            .set("intent_hash", &event.intent_hash)
            .set("challenger", &event.challenger)
            .set("block_number", event.block_number)
            .set("tx_hash", &event.tx_hash)
            .set("timestamp", event.timestamp);
    }

    Ok(tables.to_entity_changes())
}

// Event decoding functions
fn decode_intent_created_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentCreated> {
    if log.topics.len() < 4 || log.data.len() < 64 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let creator = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let source_chain_id = u64::from_be_bytes(log.topics[3][24..].try_into().ok()?);
    
    // Decode data field for destination_chain_id, reward_token, reward_amount
    let destination_chain_id = u64::from_be_bytes(log.data[24..32].try_into().ok()?);
    let reward_token = format!("0x{}", hex::encode(&log.data[44..64]));
    let reward_amount = format!("0x{}", hex::encode(&log.data[64..96]));

    Some(IntentCreated {
        intent_hash,
        creator,
        source_chain_id,
        destination_chain_id,
        reward_token,
        reward_amount,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_funded_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentFunded> {
    if log.topics.len() < 3 || log.data.len() < 64 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let funder = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let reward_token = format!("0x{}", hex::encode(&log.data[12..32]));
    let reward_amount = format!("0x{}", hex::encode(&log.data[32..64]));

    Some(IntentFunded {
        intent_hash,
        funder,
        reward_token,
        reward_amount,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_partially_funded_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentPartiallyFunded> {
    if log.topics.len() < 3 || log.data.len() < 64 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let funder = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let reward_token = format!("0x{}", hex::encode(&log.data[12..32]));
    let reward_amount = format!("0x{}", hex::encode(&log.data[32..64]));

    Some(IntentPartiallyFunded {
        intent_hash,
        funder,
        reward_token,
        reward_amount,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_withdrawal_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<Withdrawal> {
    if log.topics.len() < 3 || log.data.len() < 64 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let claimant = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let reward_token = format!("0x{}", hex::encode(&log.data[12..32]));
    let reward_amount = format!("0x{}", hex::encode(&log.data[32..64]));

    Some(Withdrawal {
        intent_hash,
        claimant,
        reward_token,
        reward_amount,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_refund_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<Refund> {
    if log.topics.len() < 3 || log.data.len() < 64 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let recipient = format!("0x{}", hex::encode(&log.topics[2][12..]));
    let reward_token = format!("0x{}", hex::encode(&log.data[12..32]));
    let reward_amount = format!("0x{}", hex::encode(&log.data[32..64]));

    Some(Refund {
        intent_hash,
        recipient,
        reward_token,
        reward_amount,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}

fn decode_intent_proof_challenged_event(log: &eth::Log, block_number: u64, tx_hash: &str, timestamp: u64) -> Option<IntentProofChallenged> {
    if log.topics.len() < 3 {
        return None;
    }

    let intent_hash = format!("0x{}", hex::encode(&log.topics[1]));
    let challenger = format!("0x{}", hex::encode(&log.topics[2][12..]));

    Some(IntentProofChallenged {
        intent_hash,
        challenger,
        block_number,
        tx_hash: tx_hash.to_string(),
        timestamp,
    })
}