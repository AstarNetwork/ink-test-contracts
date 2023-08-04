// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Contract for cross-VM payable calls test.
//! 
//! This contract can call an EVM contract via `pallet-xvm` via Chain Extension.

#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::env::{chain_extension::FromStatusCode, DefaultEnvironment, Environment};
use ink::prelude::vec::Vec;

#[ink::contract(env = CustomEnvironment)]
mod call_xvm_payable {
    use super::*;

    #[ink(storage)]
    pub struct CallXvmPayable {}

    impl CallXvmPayable {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable, selector = 42)]
        pub fn call_xvm_payable(
            &self,
            target: Vec<u8>,
            input: Vec<u8>,
        ) -> Result<Vec<u8>, XvmCallError> {
            let value = Self::env().transferred_value();
            // Calling EVM
            Self::env().extension().xvm_call(0x0F, target, input, value)
        }
    }
}

#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmCallError {
    InvalidVmId,
    SameVmCallNotAllowed,
    InvalidTarget,
    InputTooLarge,
    BadOrigin,
    ExecutionFailed,
}
impl FromStatusCode for XvmCallError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::InvalidVmId),
            2 => Err(Self::SameVmCallNotAllowed),
            3 => Err(Self::InvalidTarget),
            4 => Err(Self::InputTooLarge),
            5 => Err(Self::BadOrigin),
            6 => Err(Self::ExecutionFailed),
            _ => panic!("unknown status code"),
        }
    }
}
impl From<scale::Error> for XvmCallError {
    fn from(_: scale::Error) -> Self {
        panic!("unexpected invalid SCALE encoding")
    }
}

#[ink::chain_extension]
pub trait XvmCall {
    type ErrorCode = XvmCallError;

    #[ink(extension = 0x00010001, handle_status = false)]
    fn xvm_call(
        vm_id: u8,
        target: Vec<u8>,
        input: Vec<u8>,
        value: u128,
    ) -> Result<Vec<u8>, XvmCallError>;
}

pub enum CustomEnvironment {}
impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = XvmCall;
}
