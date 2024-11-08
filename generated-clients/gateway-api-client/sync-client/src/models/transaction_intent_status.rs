/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionIntentStatus : A more specific intent status. See the description field for further information. Note that `CommitPendingOutcomeUnknown` can either result in `CommittedSuccess` or `CommittedFailure`. 
/// A more specific intent status. See the description field for further information. Note that `CommitPendingOutcomeUnknown` can either result in `CommittedSuccess` or `CommittedFailure`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionIntentStatus {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "CommittedSuccess")]
    CommittedSuccess,
    #[serde(rename = "CommittedFailure")]
    CommittedFailure,
    #[serde(rename = "CommitPendingOutcomeUnknown")]
    CommitPendingOutcomeUnknown,
    #[serde(rename = "PermanentlyRejected")]
    PermanentlyRejected,
    #[serde(rename = "LikelyButNotCertainRejection")]
    LikelyButNotCertainRejection,
    #[serde(rename = "Pending")]
    Pending,

}

impl std::fmt::Display for TransactionIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::CommittedSuccess => write!(f, "CommittedSuccess"),
            Self::CommittedFailure => write!(f, "CommittedFailure"),
            Self::CommitPendingOutcomeUnknown => write!(f, "CommitPendingOutcomeUnknown"),
            Self::PermanentlyRejected => write!(f, "PermanentlyRejected"),
            Self::LikelyButNotCertainRejection => write!(f, "LikelyButNotCertainRejection"),
            Self::Pending => write!(f, "Pending"),
        }
    }
}

impl Default for TransactionIntentStatus {
    fn default() -> TransactionIntentStatus {
        Self::Unknown
    }
}

