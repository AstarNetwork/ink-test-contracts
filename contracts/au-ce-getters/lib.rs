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

mod helper;

use helper::UAExtension as _UAExtension;
use ink::env::DefaultEnvironment;
use sp_core::H160;

type UAExtension = _UAExtension<DefaultEnvironment>;

#[ink::contract]
mod call_xvm_payable {
    use super::*;

    #[ink(storage)]
    pub struct UAMappingGetter;

    impl UAMappingGetter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable, selector = 42)]
        pub fn to_h160(&self, account_id: AccountId) -> Option<H160> {
            UAExtension::to_h160(account_id)
        }

        #[ink(message, payable, selector = 43)]
        pub fn to_h160_or_default(&self, account_id: AccountId) -> H160 {
            UAExtension::to_h160_or_default(account_id)
        }

        #[ink(message, payable, selector = 44)]
        pub fn to_account_id(&self, evm_address: H160) -> Option<AccountId> {
            UAExtension::to_account_id(evm_address)
        }

        #[ink(message, payable, selector = 45)]
        pub fn to_account_id_or_default(&self, evm_address: H160) -> AccountId {
            UAExtension::to_account_id_or_default(evm_address)
        }
    }
}
