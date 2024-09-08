use std::fs::Metadata;
use std::iter::Map;
use std::sync::Arc;
use bitcoinsv::bitcoin::{Address, Outpoint, TxHash};

/// A 1satordinal token. This is an NFT token that is stored on-chain in transactions.
///
/// This struct collects 1satordinal actions and presents the latest known state. It also provides
/// access to the historical actions.
///
/// The possible actions that can be taken on an Ordinal are:
///  * the initial [OrdinalInscription] which defines and creates the token and assigns it to an initial [Address],
///  * additional [OrdinalTranscription]s which can update the token data and transfer the token to a new [Address],
///  * [OrdinalTransfer]s which transfer control to a new [Address].
///
/// todo: At the moment we only consider standard P2PKH control scripts.
///
/// See also [1satordinals.com](https://1satordinals.com/).
pub struct Ordinal {
    // todo: implement
}

/// An OrdinalInscription is token data stored on-chain. It is used to define a token and assign its initial
/// control [Address] or update a token and its control [Address].
///
/// This struct contains data related to a single inscription. It does not contain any of the historical
/// or future actions of a token.
pub struct OrdinalInscription {
    /// The identifier of the inscription, which is an [Outpoint].
    pub id: Outpoint,
    /// The inscription may be an update to an existing token.
    pub prev_id: Option<Outpoint>,
    /// The inscription assigns the token to an [Address].
    pub new_address: Address,
    /// Creation data. The Ordinals specification defines even numbered fields as creation, initial
    /// assignment, or transfer fields.
    pub creation_data: Map<u64, [u8]>,
    /// Metadata. The Ordinals specification defines odd numbered fields as metadata fields.
    pub metadata: Map<u64, [u8]>,
}


/// A transfer of control of an [Ordinal] from one [Address] to another.
///
/// Note that this is not necessarily the latest transfer of the token.
pub struct OrdinalTransfer {
    /// The identifier of the transfer, which is an [Outpoint].
    pub id: Outpoint,
    /// The id of the previous transfer or [OrdinalInscription].
    pub prev_id: Outpoint,
    /// The address to which the [Ordinal] has been assigned.
    pub new_address: Address,
}


