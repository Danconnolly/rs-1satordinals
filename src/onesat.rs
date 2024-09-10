
/// A 1SatOrdinal token. This is an NFT token that is stored on-chain in transactions.
///
/// This struct collects 1SatOrdinal actions and presents the latest known state. It also provides
/// access to the historical actions.
///
/// The possible actions that can be taken on an Ordinal are:
///  * the initial [OrdinalInscription] which defines and creates the token and assigns it to an initial [Address],
///  * additional [OrdinalTranscription]s which can update the token data and transfer the token to a new [Address],
///  * [OrdinalTransfer]s which transfer control to a new [Address].
///
/// Each 1SatOrdinal action has requirements to be considered valid and must be directly descended from a previous
/// 1SatOrdinal action, with the exception of the initial action which creates the token.
///
/// todo: At the moment we only consider standard P2PKH control scripts.
///
/// See also [1satordinals.com](https://1satordinals.com/).
#[derive(Debug, Clone)]
pub struct Ordinal {
    // todo: implement
}




