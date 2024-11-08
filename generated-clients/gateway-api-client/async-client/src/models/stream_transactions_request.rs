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
pub struct StreamTransactionsRequest {
    #[serde(rename = "at_ledger_state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub at_ledger_state: Option<Option<Box<models::LedgerStateSelector>>>,
    #[serde(rename = "from_ledger_state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_ledger_state: Option<Option<Box<models::LedgerStateSelector>>>,
    /// This cursor allows forward pagination, by providing the cursor from the previous request.
    #[serde(rename = "cursor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<String>>,
    /// The page size requested.
    #[serde(rename = "limit_per_page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limit_per_page: Option<Option<i32>>,
    /// Allows specifying an array of account addresses. If specified, the response will contain only transactions that, for all specified accounts, contain manifest method calls to that account which require the owner role. See the [account docs](https://docs.radixdlt.com/docs/account) for more information.
    #[serde(rename = "accounts_with_manifest_owner_method_calls", skip_serializing_if = "Option::is_none")]
    pub accounts_with_manifest_owner_method_calls: Option<Vec<String>>,
    /// Allows specifying an array of account addresses. If specified, the response will contain only transactions that, for all specified accounts, do NOT contain manifest method calls to that account which require owner role. See the [account docs](https://docs.radixdlt.com/docs/account) for more information.
    #[serde(rename = "accounts_without_manifest_owner_method_calls", skip_serializing_if = "Option::is_none")]
    pub accounts_without_manifest_owner_method_calls: Option<Vec<String>>,
    /// Allows specifying an array of global addresses. If specified, the response will contain transactions that affected all of the given global entities. A global entity is marked as \"affected\" by a transaction if any of its state (or its descendents' state) was modified as a result of the transaction. For performance reasons consensus manager and transaction tracker are excluded from that filter.
    #[serde(rename = "affected_global_entities_filter", skip_serializing_if = "Option::is_none")]
    pub affected_global_entities_filter: Option<Vec<String>>,
    /// Allows specifying an array of global addresses. If specified, the response will contain transactions in which all entities emitted events. If an event was published by an internal entity, it is going to be indexed as it is a global ancestor. For performance reasons events published by consensus manager and native XRD resource are excluded from that filter.
    #[serde(rename = "event_global_emitters_filter", skip_serializing_if = "Option::is_none")]
    pub event_global_emitters_filter: Option<Vec<String>>,
    /// Filters the transaction stream to transactions which emitted at least one event matching each filter (each filter can be satisfied by a different event). Currently *only* deposit and withdrawal events emitted by an internal vault entity are tracked. For the purpose of filtering, the emitter address is replaced by the global ancestor of the emitter, for example, the top-level account / component which contains the vault which emitted the event.
    #[serde(rename = "events_filter", skip_serializing_if = "Option::is_none")]
    pub events_filter: Option<Vec<models::StreamTransactionsRequestEventFilterItem>>,
    /// Limit returned transactions by their kind. Defaults to `user`.
    #[serde(rename = "kind_filter", skip_serializing_if = "Option::is_none")]
    pub kind_filter: Option<KindFilter>,
    /// Similar to `manifest_accounts_withdrawn_from_filter`, but will return only transactions with a manifest containing deposits to the given accounts.
    #[serde(rename = "manifest_accounts_deposited_into_filter", skip_serializing_if = "Option::is_none")]
    pub manifest_accounts_deposited_into_filter: Option<Vec<String>>,
    /// Allows specifying an array of account addresses. If specified, the response will contain only transactions with a manifest containing withdrawals from the given accounts.
    #[serde(rename = "manifest_accounts_withdrawn_from_filter", skip_serializing_if = "Option::is_none")]
    pub manifest_accounts_withdrawn_from_filter: Option<Vec<String>>,
    /// Allows specifying array of badge resource addresses. If specified, the response will contain only transactions where the given badges were presented.
    #[serde(rename = "manifest_badges_presented_filter", skip_serializing_if = "Option::is_none")]
    pub manifest_badges_presented_filter: Option<Vec<String>>,
    #[serde(rename = "manifest_class_filter", skip_serializing_if = "Option::is_none")]
    pub manifest_class_filter: Option<Box<models::StreamTransactionsRequestAllOfManifestClassFilter>>,
    /// Allows specifying array of resource addresses. If specified, the response will contain only transactions containing the given resources in the manifest (regardless of their usage).
    #[serde(rename = "manifest_resources_filter", skip_serializing_if = "Option::is_none")]
    pub manifest_resources_filter: Option<Vec<String>>,
    #[serde(rename = "opt_ins", skip_serializing_if = "Option::is_none")]
    pub opt_ins: Option<Box<models::TransactionDetailsOptIns>>,
    /// Configures the order of returned result set. Defaults to `desc`.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl StreamTransactionsRequest {
    pub fn new() -> StreamTransactionsRequest {
        StreamTransactionsRequest {
            at_ledger_state: None,
            from_ledger_state: None,
            cursor: None,
            limit_per_page: None,
            accounts_with_manifest_owner_method_calls: None,
            accounts_without_manifest_owner_method_calls: None,
            affected_global_entities_filter: None,
            event_global_emitters_filter: None,
            events_filter: None,
            kind_filter: None,
            manifest_accounts_deposited_into_filter: None,
            manifest_accounts_withdrawn_from_filter: None,
            manifest_badges_presented_filter: None,
            manifest_class_filter: None,
            manifest_resources_filter: None,
            opt_ins: None,
            order: None,
        }
    }
}
/// Limit returned transactions by their kind. Defaults to `user`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KindFilter {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "EpochChange")]
    EpochChange,
    #[serde(rename = "All")]
    All,
}

impl Default for KindFilter {
    fn default() -> KindFilter {
        Self::User
    }
}
/// Configures the order of returned result set. Defaults to `desc`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "Asc")]
    Asc,
    #[serde(rename = "Desc")]
    Desc,
}

impl Default for Order {
    fn default() -> Order {
        Self::Asc
    }
}

