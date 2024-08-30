use bitcoin::{
    absolute::LockTime, psbt::Input, transaction::Version, Amount, OutPoint, ScriptBuf, Sequence,
    Transaction, TxIn, TxOut, Witness,
};

mod btc;

pub struct TxBuilder {
    inputs: Vec<Input>,
}

impl TxBuilder {
    fn new() -> Self {
        TxBuilder { inputs: Vec::new() }
    }

    fn build() {
        let script_pubkey = ScriptBuf::new();
        let script_sig = ScriptBuf::new();
        let outpoint = OutPoint {
            ..Default::default()
        };

        let tx = Transaction {
            version: Version::TWO,
            lock_time: LockTime::ZERO,
            input: vec![TxIn {
                script_sig: script_sig,
                previous_output: outpoint,
                witness: Witness::default(),
                sequence: Sequence {
                    ..Default::default()
                },
            }],
            output: vec![TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: script_pubkey,
            }],
        };
    }
}
