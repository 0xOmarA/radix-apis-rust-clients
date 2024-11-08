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

/// AccountDepositPreValidationDecidingFactors : Deciding factors used to calculate response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDepositPreValidationDecidingFactors {
    #[serde(rename = "default_deposit_rule")]
    pub default_deposit_rule: models::AccountDefaultDepositRule,
    /// Whether the input badge belongs to the account's set of authorized depositors. This field will only be present if any badge was passed in the request.
    #[serde(rename = "is_badge_authorized_depositor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_badge_authorized_depositor: Option<Option<bool>>,
    /// Returns deciding factors for each resource. Contains only information about resources presented in the request, not all resource preference rules for queried account.
    #[serde(rename = "resource_specific_details", skip_serializing_if = "Option::is_none")]
    pub resource_specific_details: Option<Vec<models::AccountDepositPreValidationDecidingFactorsResourceSpecificDetailsItem>>,
}

impl AccountDepositPreValidationDecidingFactors {
    /// Deciding factors used to calculate response.
    pub fn new(default_deposit_rule: models::AccountDefaultDepositRule) -> AccountDepositPreValidationDecidingFactors {
        AccountDepositPreValidationDecidingFactors {
            default_deposit_rule,
            is_badge_authorized_depositor: None,
            resource_specific_details: None,
        }
    }
}

