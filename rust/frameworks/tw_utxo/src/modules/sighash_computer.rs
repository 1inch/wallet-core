// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::script::Script;
use crate::signing_mode::SigningMethod;
use crate::transaction::transaction_interface::TransactionInterface;
use crate::transaction::transaction_parts::Amount;
use crate::transaction::unsigned_transaction::UnsignedTransaction;
use crate::transaction::{TransactionPreimage, UtxoPreimageArgs, UtxoTaprootPreimageArgs};
use std::marker::PhantomData;
use tw_coin_entry::error::prelude::SigningResult;
use tw_hash::H256;

#[derive(Debug, Clone)]
pub struct TxPreimage {
    /// Transaction signatures in the same order as the transaction UTXOs.
    pub sighashes: Vec<UtxoSighash>,
}

#[derive(Debug, Clone)]
pub struct UtxoSighash {
    /// The signing method needs to be used for this sighash.
    pub signing_method: SigningMethod,
    pub sighash: H256,
}

/// Sighash Computer with a standard Bitcoin behaviour.
///
/// # Important
///
/// If needed to implement a custom logic, consider adding a different Sighash Computer.
pub struct SighashComputer<Transaction> {
    _phantom: PhantomData<Transaction>,
}

impl<Transaction> SighashComputer<Transaction>
where
    Transaction: TransactionPreimage + TransactionInterface,
{
    /// Computes sighashes of [`SighashComputer::transaction`].
    pub fn preimage_tx(
        unsigned_tx: &UnsignedTransaction<Transaction>,
    ) -> SigningResult<TxPreimage> {
        unsigned_tx
            .input_args()
            .iter()
            .enumerate()
            .map(|(input_index, utxo)| {
                let signing_method = utxo.signing_method;

                let utxo_args = UtxoPreimageArgs {
                    input_index,
                    script_pubkey: utxo.script_pubkey.clone(),
                    amount: utxo.amount,
                    // TODO move `leaf_hash_code_separator` to `UtxoTaprootPreimageArgs`.
                    leaf_hash_code_separator: utxo.leaf_hash_code_separator,
                    sighash_ty: utxo.sighash_ty,
                    tx_hasher: utxo.tx_hasher,
                    signing_method,
                };

                let sighash = match signing_method {
                    SigningMethod::Legacy | SigningMethod::Segwit => {
                        unsigned_tx.transaction().preimage_tx(&utxo_args)?
                    },
                    SigningMethod::Taproot => {
                        let tr_spent_amounts: Vec<Amount> = unsigned_tx
                            .input_args()
                            .iter()
                            .map(|utxo| utxo.amount)
                            .collect();

                        let tr_spent_script_pubkeys: Vec<Script> = unsigned_tx
                            .input_args()
                            .iter()
                            .map(|utxo| utxo.script_pubkey.clone())
                            .collect();

                        let tr = UtxoTaprootPreimageArgs {
                            args: utxo_args,
                            spent_amounts: tr_spent_amounts,
                            spent_script_pubkeys: tr_spent_script_pubkeys.clone(),
                        };

                        unsigned_tx.transaction().preimage_taproot_tx(&tr)?
                    },
                };

                Ok(UtxoSighash {
                    signing_method,
                    sighash,
                })
            })
            // Collect the results as [`SigningResult<Vec<UtxoSighash>>`].
            .collect::<SigningResult<Vec<_>>>()
            .map(|sighashes: Vec<UtxoSighash>| TxPreimage { sighashes })
    }
}
