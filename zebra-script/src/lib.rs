//! Zebra script verification wrapping zcashd's zcash_script library
#![doc(html_favicon_url = "https://zfnd.org/wp-content/uploads/2022/03/zebra-favicon-128.png")]
#![doc(html_logo_url = "https://zfnd.org/wp-content/uploads/2022/03/zebra-icon.png")]
#![doc(html_root_url = "https://docs.rs/zebra_script")]
// We allow unsafe code, so we can call zcash_script
#![allow(unsafe_code)]

use core::fmt;
use std::sync::Arc;

use thiserror::Error;

use zcash_script::ZcashScript;

use zebra_chain::{
    parameters::NetworkUpgrade,
    transaction::{HashType, SigHasher, Transaction},
    transparent,
};

/// An Error type representing the error codes returned from zcash_script.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// script verification failed
    ScriptInvalid,
    /// input index out of bounds
    TxIndex,
    /// tx is a coinbase transaction and should not be verified
    TxCoinbase,
    /// unknown error from zcash_script: {0}
    Unknown(zcash_script::Error),
    /// transaction is invalid according to zebra_chain (not a zcash_script error)
    TxInvalid(#[from] zebra_chain::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            Error::ScriptInvalid => "script verification failed".to_owned(),
            Error::TxIndex => "input index out of bounds".to_owned(),
            Error::TxCoinbase => {
                "tx is a coinbase transaction and should not be verified".to_owned()
            }
            Error::Unknown(e) => format!("unknown error from zcash_script: {e:?}"),
            Error::TxInvalid(e) => format!("tx is invalid: {e}"),
        })
    }
}

impl From<zcash_script::Error> for Error {
    #[allow(non_upper_case_globals)]
    fn from(err_code: zcash_script::Error) -> Error {
        match err_code {
            zcash_script::Error::Ok(_) => Error::ScriptInvalid,
            unknown => Error::Unknown(unknown),
        }
    }
}

/// Get the interpreter according to the feature flag
fn get_interpreter(
    sighash: zcash_script::SighashCalculator,
    lock_time: u32,
    is_final: bool,
    #[allow(unused)] flags: zcash_script::VerificationFlags,
) -> impl ZcashScript + use<'_> {
    #[cfg(feature = "comparison-interpreter")]
    return zcash_script::cxx_rust_comparison_interpreter(sighash, lock_time, is_final, flags);
    #[cfg(not(feature = "comparison-interpreter"))]
    zcash_script::CxxInterpreter {
        sighash,
        lock_time,
        is_final,
    }
}

/// A preprocessed Transaction which can be used to verify scripts within said
/// Transaction.
#[derive(Debug)]
pub struct CachedFfiTransaction {
    /// The deserialized Zebra transaction.
    ///
    /// This field is private so that `transaction`, and `all_previous_outputs` always match.
    transaction: Arc<Transaction>,

    /// The outputs from previous transactions that match each input in the transaction
    /// being verified.
    all_previous_outputs: Arc<Vec<transparent::Output>>,

    /// The sighasher context to use to compute sighashes.
    sighasher: SigHasher,
}

impl CachedFfiTransaction {
    /// Construct a `PrecomputedTransaction` from a `Transaction` and the outputs
    /// from previous transactions that match each input in the transaction
    /// being verified.
    pub fn new(
        transaction: Arc<Transaction>,
        all_previous_outputs: Arc<Vec<transparent::Output>>,
        nu: NetworkUpgrade,
    ) -> Result<Self, Error> {
        let sighasher = transaction.sighasher(nu, all_previous_outputs.clone())?;
        Ok(Self {
            transaction,
            all_previous_outputs,
            sighasher,
        })
    }

    /// Returns the transparent inputs for this transaction.
    pub fn inputs(&self) -> &[transparent::Input] {
        self.transaction.inputs()
    }

    /// Returns the outputs from previous transactions that match each input in the transaction
    /// being verified.
    pub fn all_previous_outputs(&self) -> &Vec<transparent::Output> {
        &self.all_previous_outputs
    }

    /// Return the sighasher being used for this transaction.
    pub fn sighasher(&self) -> &SigHasher {
        &self.sighasher
    }

    /// Verify if the script in the input at `input_index` of a transaction correctly spends the
    /// matching [`transparent::Output`] it refers to.
    #[allow(clippy::unwrap_in_result)]
    pub fn is_valid(&self, input_index: usize) -> Result<(), Error> {
        let previous_output = self
            .all_previous_outputs
            .get(input_index)
            .ok_or(Error::TxIndex)?
            .clone();
        let transparent::Output {
            value: _,
            lock_script,
        } = previous_output;
        let script_pub_key: &[u8] = lock_script.as_raw_bytes();

        let flags = zcash_script::VerificationFlags::P2SH
            | zcash_script::VerificationFlags::CHECKLOCKTIMEVERIFY;

        let lock_time = self.transaction.raw_lock_time();
        let is_final = self.transaction.inputs()[input_index].sequence() == u32::MAX;
        let signature_script = match &self.transaction.inputs()[input_index] {
            transparent::Input::PrevOut {
                outpoint: _,
                unlock_script,
                sequence: _,
            } => unlock_script.as_raw_bytes(),
            transparent::Input::Coinbase { .. } => Err(Error::TxCoinbase)?,
        };

        let calculate_sighash = |script_code: &[u8], hash_type: zcash_script::HashType| {
            let script_code_vec = script_code.to_vec();
            let mut our_hash_type = match hash_type.signed_outputs {
                zcash_script::SignedOutputs::All => HashType::ALL,
                zcash_script::SignedOutputs::Single => HashType::SINGLE,
                zcash_script::SignedOutputs::None => HashType::NONE,
            };
            if hash_type.anyone_can_pay {
                our_hash_type |= HashType::ANYONECANPAY;
            }
            Some(
                self.sighasher()
                    .sighash(our_hash_type, Some((input_index, script_code_vec)))
                    .0,
            )
        };
        let interpreter = get_interpreter(&calculate_sighash, lock_time, is_final, flags);
        interpreter
            .verify_callback(script_pub_key, signature_script, flags)
            .map_err(Error::from)
    }
}

/// Returns the number of transparent signature operations in the
/// transparent inputs and outputs of the given transaction.
#[allow(clippy::unwrap_in_result)]
pub fn legacy_sigop_count(transaction: &Transaction) -> Result<u64, Error> {
    let mut count: u64 = 0;

    // Create a dummy interpreter since these inputs are not used to count
    // the sigops
    let interpreter = get_interpreter(
        &|_, _| None,
        0,
        true,
        zcash_script::VerificationFlags::P2SH
            | zcash_script::VerificationFlags::CHECKLOCKTIMEVERIFY,
    );

    for input in transaction.inputs() {
        count += match input {
            transparent::Input::PrevOut {
                outpoint: _,
                unlock_script,
                sequence: _,
            } => {
                let script = unlock_script.as_raw_bytes();
                interpreter
                    .legacy_sigop_count_script(script)
                    .map_err(Error::from)?
            }
            transparent::Input::Coinbase { .. } => 0,
        } as u64;
    }

    for output in transaction.outputs() {
        let script = output.lock_script.as_raw_bytes();
        let ret = interpreter
            .legacy_sigop_count_script(script)
            .map_err(Error::from)?;
        count += ret as u64;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use hex::FromHex;
    use std::sync::Arc;
    use zebra_chain::{
        parameters::NetworkUpgrade,
        serialization::{ZcashDeserialize, ZcashDeserializeInto},
        transaction::Transaction,
        transparent::{self, Output},
    };
    use zebra_test::prelude::*;

    lazy_static::lazy_static! {
        pub static ref SCRIPT_PUBKEY: Vec<u8> = <Vec<u8>>::from_hex("76a914f47cac1e6fec195c055994e8064ffccce0044dd788ac")
            .unwrap();
        pub static ref SCRIPT_TX: Vec<u8> = <Vec<u8>>::from_hex("0400008085202f8901fcaf44919d4a17f6181a02a7ebe0420be6f7dad1ef86755b81d5a9567456653c010000006a473044022035224ed7276e61affd53315eca059c92876bc2df61d84277cafd7af61d4dbf4002203ed72ea497a9f6b38eb29df08e830d99e32377edb8a574b8a289024f0241d7c40121031f54b095eae066d96b2557c1f99e40e967978a5fd117465dbec0986ca74201a6feffffff020050d6dc0100000017a9141b8a9bda4b62cd0d0582b55455d0778c86f8628f870d03c812030000001976a914e4ff5512ffafe9287992a1cd177ca6e408e0300388ac62070d0095070d000000000000000000000000")
            .expect("Block bytes are in valid hex representation");
    }

    fn verify_valid_script(
        nu: NetworkUpgrade,
        tx: &[u8],
        amount: u64,
        pubkey: &[u8],
    ) -> Result<()> {
        let transaction =
            tx.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(pubkey),
        };
        let input_index = 0;

        let previous_output = Arc::new(vec![output]);
        let verifier = super::CachedFfiTransaction::new(transaction, previous_output, nu)
            .expect("network upgrade should be valid for tx");
        verifier.is_valid(input_index)?;

        Ok(())
    }

    #[test]
    fn verify_valid_script_v4() -> Result<()> {
        let _init_guard = zebra_test::init();

        verify_valid_script(
            NetworkUpgrade::Blossom,
            &SCRIPT_TX,
            212 * u64::pow(10, 8),
            &SCRIPT_PUBKEY,
        )
    }

    #[test]
    fn count_legacy_sigops() -> Result<()> {
        let _init_guard = zebra_test::init();

        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;

        assert_eq!(super::legacy_sigop_count(&transaction)?, 1);

        Ok(())
    }

    #[test]
    fn fail_invalid_script() -> Result<()> {
        let _init_guard = zebra_test::init();

        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;
        let coin = u64::pow(10, 8);
        let amount = 211 * coin;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(&SCRIPT_PUBKEY.clone()[..]),
        };
        let input_index = 0;
        let verifier = super::CachedFfiTransaction::new(
            transaction,
            Arc::new(vec![output]),
            NetworkUpgrade::Blossom,
        )
        .expect("network upgrade should be valid for tx");
        verifier
            .is_valid(input_index)
            .expect_err("verification should fail");

        Ok(())
    }

    #[test]
    fn reuse_script_verifier_pass_pass() -> Result<()> {
        let _init_guard = zebra_test::init();

        let coin = u64::pow(10, 8);
        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;
        let amount = 212 * coin;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(&SCRIPT_PUBKEY.clone()),
        };

        let verifier = super::CachedFfiTransaction::new(
            transaction,
            Arc::new(vec![output]),
            NetworkUpgrade::Blossom,
        )
        .expect("network upgrade should be valid for tx");

        let input_index = 0;

        verifier.is_valid(input_index)?;
        verifier.is_valid(input_index)?;

        Ok(())
    }

    #[test]
    fn reuse_script_verifier_pass_fail() -> Result<()> {
        let _init_guard = zebra_test::init();

        let coin = u64::pow(10, 8);
        let amount = 212 * coin;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(&SCRIPT_PUBKEY.clone()),
        };
        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;

        let verifier = super::CachedFfiTransaction::new(
            transaction,
            Arc::new(vec![output]),
            NetworkUpgrade::Blossom,
        )
        .expect("network upgrade should be valid for tx");

        let input_index = 0;

        verifier.is_valid(input_index)?;
        verifier
            .is_valid(input_index + 1)
            .expect_err("verification should fail");

        Ok(())
    }

    #[test]
    fn reuse_script_verifier_fail_pass() -> Result<()> {
        let _init_guard = zebra_test::init();

        let coin = u64::pow(10, 8);
        let amount = 212 * coin;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(&SCRIPT_PUBKEY.clone()),
        };
        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;

        let verifier = super::CachedFfiTransaction::new(
            transaction,
            Arc::new(vec![output]),
            NetworkUpgrade::Blossom,
        )
        .expect("network upgrade should be valid for tx");

        let input_index = 0;

        verifier
            .is_valid(input_index + 1)
            .expect_err("verification should fail");
        verifier.is_valid(input_index)?;

        Ok(())
    }

    #[test]
    fn reuse_script_verifier_fail_fail() -> Result<()> {
        let _init_guard = zebra_test::init();

        let coin = u64::pow(10, 8);
        let amount = 212 * coin;
        let output = transparent::Output {
            value: amount.try_into()?,
            lock_script: transparent::Script::new(&SCRIPT_PUBKEY.clone()),
        };
        let transaction =
            SCRIPT_TX.zcash_deserialize_into::<Arc<zebra_chain::transaction::Transaction>>()?;

        let verifier = super::CachedFfiTransaction::new(
            transaction,
            Arc::new(vec![output]),
            NetworkUpgrade::Blossom,
        )
        .expect("network upgrade should be valid for tx");

        let input_index = 0;

        verifier
            .is_valid(input_index + 1)
            .expect_err("verification should fail");

        verifier
            .is_valid(input_index + 1)
            .expect_err("verification should fail");

        Ok(())
    }

    #[test]
    fn p2sh() -> Result<()> {
        let _init_guard = zebra_test::init();

        // real tx with txid 51ded0b026f1ff56639447760bcd673b9f4e44a8afbf3af1dbaa6ca1fd241bea
        let serialized_tx = "0400008085202f8901c21354bf2305e474ad695382e68efc06e2f8b83c512496f615d153c2e00e688b00000000fdfd0000483045022100d2ab3e6258fe244fa442cfb38f6cef9ac9a18c54e70b2f508e83fa87e20d040502200eead947521de943831d07a350e45af8e36c2166984a8636f0a8811ff03ed09401473044022013e15d865010c257eef133064ef69a780b4bc7ebe6eda367504e806614f940c3022062fdbc8c2d049f91db2042d6c9771de6f1ef0b3b1fea76c1ab5542e44ed29ed8014c69522103b2cc71d23eb30020a4893982a1e2d352da0d20ee657fa02901c432758909ed8f21029d1e9a9354c0d2aee9ffd0f0cea6c39bbf98c4066cf143115ba2279d0ba7dabe2103e32096b63fd57f3308149d238dcbb24d8d28aad95c0e4e74e3e5e6a11b61bcc453aeffffffff0250954903000000001976a914a5a4e1797dac40e8ce66045d1a44c4a63d12142988acccf41c590000000017a9141c973c68b2acc6d6688eff9c7a9dd122ac1346ab8786c72400000000000000000000000000000000";
        let serialized_output = "4065675c0000000017a914c117756dcbe144a12a7c33a77cfa81aa5aeeb38187";
        let tx = Transaction::zcash_deserialize(&hex::decode(serialized_tx).unwrap().to_vec()[..])
            .unwrap();

        let previous_output =
            Output::zcash_deserialize(&hex::decode(serialized_output).unwrap().to_vec()[..])
                .unwrap();

        let verifier = super::CachedFfiTransaction::new(
            Arc::new(tx),
            Arc::new(vec![previous_output]),
            NetworkUpgrade::Nu5,
        )
        .expect("network upgrade should be valid for tx");

        verifier.is_valid(0)?;

        Ok(())
    }
}
