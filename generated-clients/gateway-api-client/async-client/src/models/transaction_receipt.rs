/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.9.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "costing_parameters", skip_serializing_if = "Option::is_none")]
    pub costing_parameters: Option<serde_json::Value>,
    /// Error message (only present if status is `Failed` or `Rejected`)
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    /// Events emitted by a transaction.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::EventsItem>>,
    /// This type is defined in the Core API as `FeeDestination`. See the Core API documentation for more details. 
    #[serde(rename = "fee_destination", skip_serializing_if = "Option::is_none")]
    pub fee_destination: Option<serde_json::Value>,
    /// This type is defined in the Core API as `FeeSource`. See the Core API documentation for more details. 
    #[serde(rename = "fee_source", skip_serializing_if = "Option::is_none")]
    pub fee_source: Option<serde_json::Value>,
    /// This type is defined in the Core API as `FeeSummary`. See the Core API documentation for more details. 
    #[serde(rename = "fee_summary", skip_serializing_if = "Option::is_none")]
    pub fee_summary: Option<serde_json::Value>,
    /// Information (number and active validator list) about new epoch if occured. This type is defined in the Core API as `NextEpoch`. See the Core API documentation for more details. 
    #[serde(rename = "next_epoch", skip_serializing_if = "Option::is_none")]
    pub next_epoch: Option<serde_json::Value>,
    /// The manifest line-by-line engine return data (only present if `status` is `CommittedSuccess`). This type is defined in the Core API as `SborData`. See the Core API documentation for more details. 
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    /// This type is defined in the Core API as `StateUpdates`. See the Core API documentation for more details. 
    #[serde(rename = "state_updates", skip_serializing_if = "Option::is_none")]
    pub state_updates: Option<serde_json::Value>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::TransactionStatus>,
}

impl TransactionReceipt {
    pub fn new() -> TransactionReceipt {
        TransactionReceipt {
            costing_parameters: None,
            error_message: None,
            events: None,
            fee_destination: None,
            fee_source: None,
            fee_summary: None,
            next_epoch: None,
            output: None,
            state_updates: None,
            status: None,
        }
    }
}

