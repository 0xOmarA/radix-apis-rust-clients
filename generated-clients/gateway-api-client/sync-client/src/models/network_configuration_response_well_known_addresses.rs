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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfigurationResponseWellKnownAddresses {
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "access_controller_package")]
    pub access_controller_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "access_rules_package")]
    pub access_rules_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "account_owner_badge")]
    pub account_owner_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "account_package")]
    pub account_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "consensus_manager")]
    pub consensus_manager: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "consensus_manager_package")]
    pub consensus_manager_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "ed25519_signature_virtual_badge")]
    pub ed25519_signature_virtual_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "faucet")]
    pub faucet: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "faucet_package")]
    pub faucet_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "genesis_helper")]
    pub genesis_helper: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "genesis_helper_package")]
    pub genesis_helper_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "global_caller_virtual_badge")]
    pub global_caller_virtual_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "identity_owner_badge")]
    pub identity_owner_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "identity_package")]
    pub identity_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "locker_package")]
    pub locker_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "metadata_module_package")]
    pub metadata_module_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "package_of_direct_caller_virtual_badge")]
    pub package_of_direct_caller_virtual_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "package_owner_badge")]
    pub package_owner_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "package_package")]
    pub package_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "pool_package")]
    pub pool_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "resource_package")]
    pub resource_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "role_assignment_module_package")]
    pub role_assignment_module_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "royalty_module_package")]
    pub royalty_module_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "secp256k1_signature_virtual_badge")]
    pub secp256k1_signature_virtual_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "system_transaction_badge")]
    pub system_transaction_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "transaction_processor_package")]
    pub transaction_processor_package: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "transaction_tracker")]
    pub transaction_tracker: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "validator_owner_badge")]
    pub validator_owner_badge: String,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "xrd")]
    pub xrd: String,
}

impl NetworkConfigurationResponseWellKnownAddresses {
    pub fn new(access_controller_package: String, access_rules_package: String, account_owner_badge: String, account_package: String, consensus_manager: String, consensus_manager_package: String, ed25519_signature_virtual_badge: String, faucet: String, faucet_package: String, genesis_helper: String, genesis_helper_package: String, global_caller_virtual_badge: String, identity_owner_badge: String, identity_package: String, locker_package: String, metadata_module_package: String, package_of_direct_caller_virtual_badge: String, package_owner_badge: String, package_package: String, pool_package: String, resource_package: String, role_assignment_module_package: String, royalty_module_package: String, secp256k1_signature_virtual_badge: String, system_transaction_badge: String, transaction_processor_package: String, transaction_tracker: String, validator_owner_badge: String, xrd: String) -> NetworkConfigurationResponseWellKnownAddresses {
        NetworkConfigurationResponseWellKnownAddresses {
            access_controller_package,
            access_rules_package,
            account_owner_badge,
            account_package,
            consensus_manager,
            consensus_manager_package,
            ed25519_signature_virtual_badge,
            faucet,
            faucet_package,
            genesis_helper,
            genesis_helper_package,
            global_caller_virtual_badge,
            identity_owner_badge,
            identity_package,
            locker_package,
            metadata_module_package,
            package_of_direct_caller_virtual_badge,
            package_owner_badge,
            package_package,
            pool_package,
            resource_package,
            role_assignment_module_package,
            royalty_module_package,
            secp256k1_signature_virtual_badge,
            system_transaction_badge,
            transaction_processor_package,
            transaction_tracker,
            validator_owner_badge,
            xrd,
        }
    }
}

