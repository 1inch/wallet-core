// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

pub type UtxoErrorKind = tw_proto::Utxo::Proto::Error;

pub trait IntoUtxoError {
    fn into_utxo(self) -> UtxoError;
}

impl IntoUtxoError for UtxoErrorKind {
    fn into_utxo(self) -> UtxoError {
        UtxoError(self)
    }
}

pub type UtxoResult<T> = Result<T, UtxoError>;

pub struct UtxoError(pub UtxoErrorKind);
