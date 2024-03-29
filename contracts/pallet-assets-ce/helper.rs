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

use core::marker::PhantomData;
use ink::env::{DefaultEnvironment, Environment};
use scale::{Encode, Decode};
use ink::prelude::vec::Vec;
use assets_chain_extension_types::Command;

/// Pallet Assets Extension Interface
pub struct AssetsExtension<E = DefaultEnvironment, const ID: u16 = 02>(PhantomData<E>);

impl<E: Environment, const ID: u16> AssetsExtension<E, ID> {
    const fn get_func_id(idx: u16) -> u32 {
        ((ID as u32) << 16) + (idx as u32)
    }

    pub fn transfer(id: u128, target: E::AccountId, amount: E::Balance) -> Result<(), AssetsError> {
        let func_id: u32 = Self::get_func_id(Command::Transfer.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(id, target, amount))
    }

    pub fn transfer_approved(id: u128, owner: E::AccountId, destination: E::AccountId, amount: E::Balance) -> Result<(), AssetsError> {
        let func_id: u32 = Self::get_func_id(Command::TransferApproved.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::AccountId, E::Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(id, owner, destination, amount))
    }

    pub fn mint(id: u128, beneficiary: E::AccountId, amount: E::Balance) -> Result<(), AssetsError> {
        let func_id: u32 = Self::get_func_id(Command::Mint.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(id, beneficiary, amount))
    }

    pub fn burn(id: u128, who: E::AccountId, amount: E::Balance) -> Result<(), AssetsError> {
        let func_id: u32 = Self::get_func_id(Command::Burn.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(id, who, amount))
    }

    pub fn approve_transfer(id: u128, delegate: E::AccountId, amount: E::Balance) -> Result<(), AssetsError> {
        let func_id: u32 = Self::get_func_id(Command::ApproveTransfer.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(id, delegate, amount))
    }

    pub fn balance_of(id: u128, who: E::AccountId) -> E::Balance {
        let func_id: u32 = Self::get_func_id(Command::BalanceOf.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId)>()
            .output::<E::Balance, false>()
            .ignore_error_code()
            .call(&(id, who))
    }

    pub fn allowance(id: u128, owner: E::AccountId, delegate: E::AccountId) -> E::Balance {
        let func_id: u32 = Self::get_func_id(Command::Allowance.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<(u128, E::AccountId, E::AccountId)>()
            .output::<E::Balance, false>()
            .ignore_error_code()
            .call(&(id, owner, delegate))
    }

    pub fn total_supply(id: u128) -> E::Balance {
        let func_id: u32 = Self::get_func_id(Command::TotalSupply.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<u128>()
            .output::<E::Balance, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_name(id: u128) -> Vec<u8> {
        let func_id: u32 = Self::get_func_id(Command::MetadataName.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_symbol(id: u128) -> Vec<u8> {
        let func_id: u32 = Self::get_func_id(Command:: MetadataSymbol.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_decimals(id: u128) -> u8 {
        let func_id: u32 = Self::get_func_id(Command::MetadataDecimals.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<u128>()
            .output::<u8, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn minimum_balance(id: u128) -> E::Balance {
        let func_id: u32 = Self::get_func_id(Command::MinimumBalance.into());

        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<u128>()
            .output::<E::Balance, false>()
            .ignore_error_code()
            .call(&id)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AssetsError {
    /// Account balance must be greater than or equal to the transfer amount.
    BalanceLow = 1,
    /// The account to alter does not exist.
    NoAccount = 2,
    /// The signing account has no permission to do the operation.
    NoPermission = 3,
    /// The given asset ID is unknown.
    Unknown = 4,
    /// The origin account is frozen.
    Frozen = 5,
    /// The asset ID is already taken.
    InUse = 6,
    /// Invalid witness data given.
    BadWitness = 7,
    /// Minimum balance should be non-zero.
    MinBalanceZero = 8,
    /// Unable to increment the consumer reference counters on the account. Either no provider
    /// reference exists to allow a non-zero balance of a non-self-sufficient asset, or one
    /// fewer then the maximum number of consumers has been reached.
    UnavailableConsumer = 9,
    /// Invalid metadata given.
    BadMetadata = 10,
    /// No approval exists that would allow the transfer.
    Unapproved = 11,
    /// The source account would not survive the transfer and it needs to stay alive.
    WouldDie = 12,
    /// The asset-account already exists.
    AlreadyExists = 13,
    /// The asset-account doesn't have an associated deposit.
    NoDeposit = 14,
    /// The operation would result in funds being burned.
    WouldBurn = 15,
    /// The asset is a live asset and is actively being used. Usually emit for operations such
    /// as `start_destroy` which require the asset to be in a destroying state.
    LiveAsset = 16,
    /// The asset is not live, and likely being destroyed.
    AssetNotLive = 17,
    /// The asset status is not the expected status.
    IncorrectStatus = 18,
    /// The asset should be frozen before the given operation.
    NotFrozen = 19,
    /// Callback action resulted in error
    CallbackFailed = 20,
    /// Unknown error
    RuntimeError = 99,
    /// Unknow status code
    UnknownStatusCode,
    /// Encountered unexpected invalid SCALE encoding
    InvalidScaleEncoding,
}

impl ink::env::chain_extension::FromStatusCode for AssetsError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::BalanceLow),
            2 => Err(Self::NoAccount),
            3 => Err(Self::NoPermission),
            4 => Err(Self::Unknown),
            5 => Err(Self::Frozen),
            6 => Err(Self::InUse),
            7 => Err(Self::BadWitness),
            8 => Err(Self::MinBalanceZero),
            9 => Err(Self::UnavailableConsumer),
            10 => Err(Self::BadMetadata),
            11 => Err(Self::Unapproved),
            12 => Err(Self::WouldDie),
            13 => Err(Self::AlreadyExists),
            14 => Err(Self::NoDeposit),
            15 => Err(Self::WouldBurn),
            16 => Err(Self::LiveAsset),
            17 => Err(Self::AssetNotLive),
            18 => Err(Self::IncorrectStatus),
            19 => Err(Self::NotFrozen),
            20 => Err(Self::CallbackFailed),
            99 => Err(Self::RuntimeError),
            _ => Err(Self::UnknownStatusCode),
        }
    }
}
impl From<scale::Error> for AssetsError {
    fn from(_: scale::Error) -> Self {
        AssetsError::InvalidScaleEncoding
    }
}