#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    use ink::storage::Mapping;
    
    #[ink(storage)]
    pub struct SimpleStorage {
        storages: Mapping<AccountId, u32>,
    }

    impl SimpleStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            let storages = Mapping::default();
            Self { storages }
        }
        
        #[ink(message, selector = 42)]
        pub fn store(&mut self) {
            let caller = self.env().caller();
            self.storages.insert(caller, &42);
        }
        
        #[ink(message, selector = 43)]
        pub fn get(&self) -> u32 {
            let caller = self.env().caller();
            self.storages.get(caller).unwrap_or(0)
        }
    }
}
