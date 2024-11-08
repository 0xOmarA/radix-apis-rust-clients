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
pub struct TransactionPreviewRequestFlags {
    /// Whether the virtual signature proofs should be automatically placed in the auth zone. 
    #[serde(rename = "assume_all_signature_proofs")]
    pub assume_all_signature_proofs: bool,
    /// Whether to skip the auth checks during execution.  This could be used to e.g.: * Preview protocol update style transactions. * Mint resources for previewing trades with resources you don't own. If doing this, be warned:   * Only resources which were potentially mintable/burnable at creation time     will be mintable/burnable, due to feature flags on the resource.   * Please see the below warning about unexpected results if using this approach.  Warning: this mode of operation is quite a departure from normal operation: * Calculated fees will likely be lower than a standard execution. * This mode can subtly break invariants some dApp code might rely on, or result in unexpected   behaviour, so the resulting execution result might not be valid for your needs. For example,   if I used this flag to mint pool units to preview a redemption (or some dApp interaction which   behind the scenes redeemed them), they'd redeem for less than they're currently worth,   because the blueprint code relies on the total supply of the pool units to calculate their   redemption worth, and you've just inflated the total supply through the mint operation. 
    #[serde(rename = "disable_auth_checks", skip_serializing_if = "Option::is_none")]
    pub disable_auth_checks: Option<bool>,
    /// Whether to skip the epoch range check (i.e. ignoring the `start_epoch_inclusive` and `end_epoch_exclusive` parameters, if specified).  Note: effectively, without an epoch range, the Radix Engine cannot perform the *intent hash duplicate* detection, which means that this check will be skipped as well. 
    #[serde(rename = "skip_epoch_check")]
    pub skip_epoch_check: bool,
    /// Whether to use a virtual, preview-only pool of XRD to pay for all execution fees. 
    #[serde(rename = "use_free_credit")]
    pub use_free_credit: bool,
}

impl TransactionPreviewRequestFlags {
    pub fn new(assume_all_signature_proofs: bool, skip_epoch_check: bool, use_free_credit: bool) -> TransactionPreviewRequestFlags {
        TransactionPreviewRequestFlags {
            assume_all_signature_proofs,
            disable_auth_checks: None,
            skip_epoch_check,
            use_free_credit,
        }
    }
}

