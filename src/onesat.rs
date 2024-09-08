use std::iter::Map;
use bitcoinsv::bitcoin::{Address, Outpoint, Tx};
use crate::result::OrdinalResult;

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
pub struct Ordinal {
    // todo: implement
}

/// An OrdinalInscription is token data stored on-chain. It is used to define a token and assign its initial
/// control [Address] or update a token and its control [Address].
///
/// This struct contains data related to a single inscription. It does not contain any of the historical
/// or future actions of a token.
///
/// An inscription can either be used to create an Ordinal or to update one. An update must link to
/// a previous inscription or ordinal transfer. Inscriptions and transfers are encoded in transaction
/// outputs and the value of the output must be one satoshi. Therefore, if the transaction that contains
/// the (output that contains the) inscription only has inputs with value > 1 then it must be a
/// creation inscription (see must_be_creation) field.
pub struct OrdinalInscription {
    /// The identifier of the inscription, which is an [Outpoint].
    pub id: Outpoint,
    /// The inscription may be an update to an existing token.
    pub prev_id: Option<Outpoint>,
    /// The inscription assigns the token to an [Address].
    pub new_address: Address,
    /// It may be possible to determine that this must be a creation inscription, as opposed to
    /// an update transaction. If so, then this field is true. If it is not known, then this field
    /// is false, but this does not imply that it is must be an update inscription or that it
    /// cannot be a creation inscription.
    pub must_be_creation: bool,
    /// Creation data. The Ordinals specification defines even numbered fields as creation, initial
    /// assignment, or transfer fields.
    pub creation_data: Map<u64, Vec<u8>>,
    /// Metadata. The Ordinals specification defines odd numbered fields as metadata fields.
    pub metadata: Map<u64, Vec<u8>>,
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


impl OrdinalInscription {
    /// Scan a transaction for inscriptions.
    ///
    /// A transaction can contain multiple transcriptions. If the transaction does not contain
    /// any inscriptions then this returns an empty Vec, so the function can be used for detection purposes.
    ///
    /// Inscriptions will be checked to see whether they are valid **within the context of the
    /// transaction**. The function cannot check if they are completely valid because that requires
    /// the transaction chain back to the original creation inscription.
    ///
    /// For the purposes of this function, an inscription is valid if it follows the conventions for
    /// data definition and the output has a value of 1 satoshi.
    ///
    /// It is not possible at this stage to examine the inputs to make validity judgements. It is
    /// possible, for example, to have a transaction with one input and two creation inscription outputs.
    /// It is also possible to have a transaction with two inputs and two inscription outputs and
    /// those inscriptions could either be update or creation inscriptions.
    ///
    /// If an inscription is invalid then it is ignored, not included in the result set, and this
    /// does not result in an error being returned.
    pub fn scan_tx(tx: &Tx) -> OrdinalResult<Vec<Self>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use bitcoinsv::bitcoin::{Encodable, Tx};
    use hex_literal::hex;
    use crate::onesat::OrdinalInscription;

    /// Scan transactions that do not contain inscriptions.
    #[test]
    fn scan_empty_tx() {
        let empty_tx = vec!(
            // coinbase tx 39b9303474b905ede8512e29939feeaf341a354391974cc4c5828befef417373
            hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1a03eb230d2f7461616c2e636f6d2fca6a8362b6eb6dfbb91a0000ffffffff01208aa012000000001976a914522cf9e7626d9bd8729e5a1398ece40dad1b6a2f88ac00000000").to_vec(),
            // certixhash tx f20ff38ab16c2861a059f47c97ca71b68db3651b480b50d318e58e74d723cf0a
            hex!("0100000001690b52787cfa5189d8e512c30f9f985f306aaa8b893483c070c26f43dfa12c722f0600006b4830450221009653a6fbf13f49eb99b50556bb93072bac6949ee93831d85f08d7eae0509937e02202daf5fe52f9528f11f7a8ab714b72f173da14f63ff3ce0455202ee10b3524ae5412103fd290068ae945c23a06775de8422ceb6010aaebab40b78e01a0af3f1322fa861ffffffff010000000000000000b1006a0963657274696861736822314c6d763150594d70387339594a556e374d3948565473446b64626155386b514e4a4031373366313139653866613830376632346631363564363863316134376563636534356363373062313030333137646334303337373436336330306232366463403630616635373462356631663333383861613839333466343533666364646138623932653232653038363238353132383832643638366335333434366365663800000000").to_vec(),
            // p2pkh tx 5b45f0397226f70e121eeade65a38a902696d9a474f3545e1fd264d53c26dc0c
            hex!("0100000001dd99d5f4ab9c10167038834624684e0a96dfe9175953d40eaaacbbfac8af6990000000006a47304402203c4923bebb1505fe92494accb563655216f61adf6d2e7a8f41d6080e6faa75c6022010612d8ea67ccd3891e6b2ada8ba9fc23dfba117008bc0caca220b49e12a7f1f4121030d4fc707a6d8a7dfecaa68e03e60a6f450ac1f7d7b73277a5e215effa6c91982ffffffff07404b4c00000000001976a914a92755206baffbc55fa8308fad18045b0569be2488ac40420f00000000001976a914d0ec882d970786bb5467ca2dd1eddb1552d18c5288ac400d0300000000001976a914a6267d9c0a9748203eb5fb039a631e8764cab6c888ac50c30000000000001976a914bdc5251eabc2128791360543b9f5ad8ee5c5b60788ac204e0000000000001976a91467b09bc3e7ba5ebc29c35769f85d387701f17df088acd0070000000000001976a9140a340974aea1f22ce7ccfeb836c0f2b044f8355188aceef20100000000001976a914fb9202a7cb63fb8fff16a84194435978f4f3a0c288ac00000000").to_vec(),
        );
        for b in empty_tx {
            let t = Tx::from_binary_buf(&b).unwrap();
            let r = OrdinalInscription::scan_tx(&t).unwrap();
            assert_eq!(0, r.len());
        }
    }
}
