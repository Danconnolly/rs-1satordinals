use std::collections::BTreeMap;
use bitcoinsv::bitcoin::{Address, FromHex, Operation, Outpoint, Tx, TxHash, TxOutput};
use bytes::Bytes;
use log::trace;
use crate::result::OrdinalResult;

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
#[derive(Debug, Clone)]
pub struct OrdinalInscription {
    /// The identifier of the inscription, which is an [Outpoint].
    pub id: Outpoint,
    /// The inscription may be an update to an existing token.
    ///
    /// If this is None then it may be that the previous token definition is not known.
    pub prev_id: Option<Outpoint>,
    /// The inscription can assign the token to a P2PKH [Address].
    pub new_address: Option<Address>,
    /// It may be possible to determine that this must be a creation inscription, as opposed to
    /// an update transaction. If so, then this field is true. If it is not known, then this field
    /// is false, but this does not imply that it is must be an update inscription or that it
    /// cannot be a creation inscription.
    pub must_be_creation: bool,
    /// Creation data. The Ordinals specification defines even numbered fields as creation, initial
    /// assignment, or transfer fields.
    pub creation_data: BTreeMap<i64, Bytes>,
    /// Metadata. The Ordinals specification defines odd numbered fields as metadata fields.
    pub metadata: BTreeMap<i64, Bytes>,
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
    /// It is not possible at this stage to examine the inputs to make validity judgements. It is
    /// possible, for example, to have a transaction with one input and two creation inscription outputs.
    /// It is also possible to have a transaction with two inputs and two inscription outputs and
    /// those inscriptions could either be update or creation inscriptions.
    ///
    /// If an inscription is invalid then it is ignored, not included in the result set, and this
    /// does not result in an error being returned.
    pub fn scan_tx(tx: &Tx) -> OrdinalResult<Vec<Self>> {
        let mut result = Vec::new();
        let mut index = 0;
        for o in &tx.outputs {
            trace!("scanning output {} of tx {}", index, tx.hash());
            match Self::scan_output(o, &tx.hash(), index) {
                None => {},
                Some(i) => {
                    trace!("found inscription {:?}", i);
                    result.push(i);
                },
            }
            index += 1;
        }
        Ok(result)
    }

    /// Scan an output for an inscription.
    ///
    /// For the purposes of this function, an inscription is valid if it follows the conventions for
    /// data definition and the output has a value of 1 satoshi.
    pub fn scan_output(txo: &TxOutput, tx_id: &TxHash, index: u32) -> Option<Self> {
        if txo.value != 1  {
            None
        } else {
            match txo.script.decode() {
                Ok((ops, trailing)) => {
                    let mut creation_data = BTreeMap::new();
                    let mut metadata = BTreeMap::new();
                    let mut key = 0i64;
                    enum State { Initial, OpFalseSeen, OpIfSeen, InEnvelope, GotKey, GotBody }
                    let mut state = State::Initial;
                    for op in ops {
                        match state {
                            State::Initial => {
                                // looking for initial OP_FALSE
                                if op.eq_alias(&Operation::OP_FALSE) {
                                    state = State::OpFalseSeen;
                                    trace!("found OP_FALSE");
                                }
                            },
                            State::OpFalseSeen => {
                                // next op must be OP_IF
                                if op == Operation::OP_IF {
                                    state = State::OpIfSeen;
                                    trace!("found OP_IF");
                                } else {
                                    state = State::Initial;
                                }
                            },
                            State::OpIfSeen => {
                                // next must be "ord" on stack
                                match op.data_pushed() {
                                    None => { state = State::Initial; },
                                    Some(d) => {
                                        trace!("found data push after OP_IF");
                                        if d.len() != 3 {
                                            state = State::Initial;
                                        } else if d.slice(0..3) == "ord" {
                                            // reset all collected data so far
                                            creation_data = BTreeMap::new();
                                            metadata = BTreeMap::new();
                                            key = 0;
                                            state = State::InEnvelope;
                                            trace!("in 1satordinal envelope");
                                        } else {
                                            state = State::Initial;
                                        }
                                    }
                                }
                            },
                            State::InEnvelope => {
                                // in the envelope, next must be a key
                                if op.is_data_push() {
                                    if let Some(v) = op.small_num_pushed() {
                                        key = v;
                                        state = State::GotKey;
                                        trace!("got key {}", key);
                                    } else {
                                        // its not valid, go back to beginning
                                        state = State::Initial;
                                    }
                                } else {
                                    // its not valid, go back to beginning
                                    state = State::Initial;
                                }
                            },
                            State::GotKey => {
                                // got a key, next must be a value
                                if op.is_data_push() {
                                    let d = op.data_pushed()?;
                                    trace!("got value, length {}", d.len());
                                    if key % 2 == 0 {
                                        metadata.insert(key, d);
                                    } else {
                                        creation_data.insert(key, d);
                                    }
                                    if key == 0 {
                                        state = State::GotBody;
                                    } else {
                                        state = State::InEnvelope;
                                    }
                                } else {
                                    // its not valid, go back to the beginning
                                    state = State::Initial;
                                }
                            },
                            State::GotBody => {
                                // we found the body, now close the envelope
                                if op != Operation::OP_ENDIF {
                                    // if its not followed by an OP_ENDIF then its invalid
                                    state = State::Initial;
                                } else {
                                    return Some(Self {
                                        id: Outpoint {
                                            tx_hash: tx_id.clone(),
                                            index,
                                        },
                                        prev_id: None,          // can't determine this from the output
                                        new_address: None,      // todo: address detection
                                        must_be_creation: false,
                                        creation_data,
                                        metadata,
                                    })
                                }
                            },
                        }
                    }
                    None
                },
                Err(err) => {
                    trace!("error decoding script, ignoring, error={:?}", err);
                    None
                }   // ignore scripts with errors
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoinsv::bitcoin::{AsyncEncodable, FromHex, Tx};
    use hex_literal::hex;

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

    /// Scan a mainnet transaction
    #[test]
    fn scan_tx() {
        // https://whatsonchain.com/tx/1fefad9e727d1e520c27372a12791c7d31ca9be933f46e92eb61da8e14ba2f6d?tab=lpw8ity1
        //        Some(OrdinalInscription { id: Outpoint { tx_hash: 1fefad9e727d1e520c27372a12791c7d31ca9be933f46e92eb61da8e14ba2f6d, index: 1 }, prev_id: None, new_address: None, must_be_creation: false, creation_data: {1: b"application/bsv-20"}, metadata: {0: b"{\"p\":\"bsv-20\",\"op\":\"transfer\",\"amt\":\"2864387\",\"tick\":\"LOL\"}"} })
        //        Some(OrdinalInscription { id: Outpoint { tx_hash: 1fefad9e727d1e520c27372a12791c7d31ca9be933f46e92eb61da8e14ba2f6d, index: 0 }, prev_id: None, new_address: None, must_be_creation: false, creation_data: {1: b"application/bsv-20"}, metadata: {0: b"{\"p\":\"bsv-20\",\"op\":\"transfer\",\"amt\":\"25\",\"tick\":\"LOL\"}"} })
        let tx = Tx::from_hex("010000000288e9ce76cb52d0c845272d1688ea510d19cf59cb692a212ff2d5438f063cb441010000006b483045022100c3b7e1c067eca9741a8f74795c07743711b5f98070144b6d02d1875f7859652902202796066e482e5689101cb628bede30eb6898071d8900b459077ece2bf71e3ae2c121033ae28579dc1a189b1e7eef911ee9f18b914644b5dd9d00a4032a894ad8fb014fffffffff88e9ce76cb52d0c845272d1688ea510d19cf59cb692a212ff2d5438f063cb441030000006b483045022100c295812032c5b9778a6a093396cd29b0e427ea437c94c56834071b0a221a8d91022060acb00bad0a71b24d0879deb90ad2f894f9d43cd7021891f982f0ded85d2b85c1210288d08f20ccf5a908668160a8d0173f688f5d43fad9b7f8c33683b349c499154bffffffff0401000000000000006c0063036f726451126170706c69636174696f6e2f6273762d323000367b2270223a226273762d3230222c226f70223a227472616e73666572222c22616d74223a223235222c227469636b223a224c4f4c227d6876a914098ed6d96b6718444a39d9f27d9a3a6ab8200e9a88ac0100000000000000710063036f726451126170706c69636174696f6e2f6273762d3230003b7b2270223a226273762d3230222c226f70223a227472616e73666572222c22616d74223a2232383634333837222c227469636b223a224c4f4c227d6876a914ebccfc5b92b0345db0fcd3dba71ccd2464ce29b088acd0070000000000001976a9142bdf72063d9a16b7d642c0825577d957bd85c93b88ace0382b00000000001976a914099fde5ce081bd5c0b3b6ef84fcfcd7fae8a3f9b88ac00000000").unwrap();
        let os = OrdinalInscription::scan_tx(&tx).unwrap();
        assert_eq!(2, os.len());
    }
}