/*
 * Radix Core API
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionIdentifiers {
    /// The hex-encoded intent hash for a user transaction, also known as the transaction id. This hash identifies the core content \"intent\" of the transaction. Each intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent. 
    #[serde(rename = "intent_hash")]
    pub intent_hash: String,
    /// The Bech32m-encoded human readable `IntentHash`.
    #[serde(rename = "intent_hash_bech32m")]
    pub intent_hash_bech32m: String,
    /// The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature. 
    #[serde(rename = "payload_hash")]
    pub payload_hash: String,
    /// The Bech32m-encoded human readable `NotarizedTransactionHash`.
    #[serde(rename = "payload_hash_bech32m")]
    pub payload_hash_bech32m: String,
    /// The hex-encoded signed intent hash for a user transaction. This hash identifies the transaction intent, plus additional signatures. This hash is signed by the notary, to create the submittable NotarizedTransaction. 
    #[serde(rename = "signed_intent_hash")]
    pub signed_intent_hash: String,
    /// The Bech32m-encoded human readable `SignedIntentHash`.
    #[serde(rename = "signed_intent_hash_bech32m")]
    pub signed_intent_hash_bech32m: String,
}

impl TransactionIdentifiers {
    pub fn new(intent_hash: String, intent_hash_bech32m: String, payload_hash: String, payload_hash_bech32m: String, signed_intent_hash: String, signed_intent_hash_bech32m: String) -> TransactionIdentifiers {
        TransactionIdentifiers {
            intent_hash,
            intent_hash_bech32m,
            payload_hash,
            payload_hash_bech32m,
            signed_intent_hash,
            signed_intent_hash_bech32m,
        }
    }
}

