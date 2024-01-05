#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod dummy_error {

    #[ink(storage)]
    pub struct DummyError {}
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        #[codec(index = 7)]
        DummyError,
    }

    impl DummyError {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
        
        #[ink(message, selector = 42)]
        pub fn do_revert(&self) -> Result<(), Error> {
            Err(Error::DummyError)
        }
    }
}
