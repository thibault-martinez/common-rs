// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::{
    error::{UnknownTagError, UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

#[derive(Debug)]
pub struct PickyError(u8);

impl From<Infallible> for PickyError {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

fn verify_value<const VERIFY: bool>(&value: &u8, _: &()) -> Result<(), PickyError> {
    if !VERIFY || value == 42 {
        Ok(())
    } else {
        Err(PickyError(value))
    }
}

#[derive(Packable)]
#[packable(unpack_error = PickyError)]
#[packable(unpack_visitor = ())]
pub struct Picky(#[packable(verify_with = verify_value)] u8);

fn main() {}
