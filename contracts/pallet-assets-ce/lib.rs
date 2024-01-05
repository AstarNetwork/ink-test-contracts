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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod helper;

use helper::AssetsExtension as _AssetsExtension;
use ink::env::DefaultEnvironment;
use crate::helper::AssetsError;
use ink::prelude::vec::Vec;

type AssetsExtension = _AssetsExtension<DefaultEnvironment>;

#[ink::contract]
pub mod contract {
    use super::*;

    #[ink(storage)]
    pub struct Mock;

    impl Mock {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn mint(
            &mut self,
            asset_id: u128,
            beneficiary: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::mint(asset_id, beneficiary, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn(
            &mut self,
            asset_id: u128,
            who: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::burn(asset_id, who, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            asset_id: u128,
            target: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::transfer(asset_id, target, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn approve_transfer(
            &mut self,
            asset_id: u128,
            delegate: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::approve_transfer(asset_id, delegate, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_approved(
            &mut self,
            asset_id: u128,
            owner: AccountId,
            destination: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::transfer_approved(
                asset_id,
                owner,
                destination,
                amount,
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, asset_id: u128, who: AccountId) -> Balance {
            AssetsExtension::balance_of(asset_id, who)
        }

        #[ink(message)]
        pub fn total_supply(&self, asset_id: u128) -> Balance {
            AssetsExtension::total_supply(asset_id)
        }

        #[ink(message)]
        pub fn allowance(&self, asset_id: u128, owner: AccountId, delegate: AccountId) -> Balance {
            AssetsExtension::allowance(asset_id, owner, delegate)
        }

        #[ink(message)]
        pub fn metadata_name(&self, asset_id: u128) -> Vec<u8> {
            AssetsExtension::metadata_name(asset_id)
        }

        #[ink(message)]
        pub fn metadata_symbol(&self, asset_id: u128) -> Vec<u8> {
            AssetsExtension::metadata_symbol(asset_id)
        }

        #[ink(message)]
        pub fn metadata_decimals(&self, asset_id: u128) -> u8 {
            AssetsExtension::metadata_decimals(asset_id)
        }

        #[ink(message)]
        pub fn minimum_balance(&self, asset_id: u128) -> Balance {
            AssetsExtension::minimum_balance(asset_id)
        }
    }
}