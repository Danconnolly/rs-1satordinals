use bitcoinsv::bitcoin::{Address, Outpoint};


/// A transfer of control of an [Ordinal] from one [Address] to another.
///
/// Note that this is not necessarily the latest transfer of the token.
#[derive(Debug, Clone)]
pub struct OrdinalTransfer {
    /// The identifier of the transfer, which is an [Outpoint].
    pub id: Outpoint,
    /// The id of the previous transfer or [OrdinalInscription].
    pub prev_id: Outpoint,
    /// The address to which the [Ordinal] has been assigned.
    pub new_address: Address,
}
