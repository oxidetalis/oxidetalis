// OxideTalis Messaging Protocol homeserver core implementation
// Copyright (C) 2024 Awiteb <a@4rs.nl>, OxideTalis Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Implemented SeaORM support for core types, enabling the use of these types
//! as column types in SeaORM

use sea_orm::{
    sea_query::{ArrayType, BlobSize, ValueType, ValueTypeErr},
    ColumnType,
    DbErr,
    QueryResult,
    TryGetError,
    TryGetable,
    Value,
};

use super::PublicKey;

impl From<PublicKey> for Value {
    fn from(public_key: PublicKey) -> Self {
        public_key.as_bytes().as_slice().into()
    }
}

impl From<&PublicKey> for Value {
    fn from(public_key: &PublicKey) -> Self {
        public_key.as_bytes().as_slice().into()
    }
}

impl TryGetable for PublicKey {
    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        let db_err = |err: &str| TryGetError::DbErr(DbErr::Type(err.to_owned()));

        <Vec<u8> as TryGetable>::try_get_by(res, idx).and_then(|v| {
            v.try_into()
                .map_err(|_| db_err("Invalid binary length"))
                .and_then(|bytes| {
                    <PublicKey as TryFrom<[u8; 33]>>::try_from(bytes)
                        .map_err(|_| db_err("Invalid Public Key"))
                })
        })
    }
}

impl ValueType for PublicKey {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        <Vec<u8> as ValueType>::try_from(v).and_then(|v| {
            v.try_into().map_err(|_| ValueTypeErr).and_then(|bytes| {
                <PublicKey as TryFrom<[u8; 33]>>::try_from(bytes).map_err(|_| ValueTypeErr)
            })
        })
    }

    fn type_name() -> String {
        String::from("PublicKey")
    }

    fn array_type() -> ArrayType {
        ArrayType::Bytes
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(BlobSize::Blob(None))
    }
}
