#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod survey {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Survey {
        /// Stores a single `u32` value on the storage.
        value: u32,
    }

    impl Survey {
        /// Constructor that initializes the `u32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `u32` value to `0`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `u32` from `0`
        /// to `0` and vice versa.
        #[ink(message)]
        pub fn set(&mut self, value: u32) {
            self.value = value;
        }

        /// Simply returns the current value of our `u32`.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let survey = Survey::default();
            assert_eq!(survey.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut survey = Survey::new(0);
            assert_eq!(survey.get(), 0);
            survey.set(0);
            assert_eq!(survey.get(), 0);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult = std::result::Result<u32, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = SurveyRef::default();

            // When
            let contract_account_id = client
                .instantiate("survey", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<SurveyRef>(contract_account_id.clone())
                .call(|survey| survey.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), 0));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = SurveyRef::new(0);
            let contract_account_id = client
                .instantiate("survey", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<SurveyRef>(contract_account_id.clone())
                .call(|survey| survey.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), 0));

            // When
            let set = build_message::<SurveyRef>(contract_account_id.clone())
                .call(|survey| survey.set());
            let _flip_result = client
                .call(&ink_e2e::bob(), set, 0, None)
                .await
                .expect("set failed");

            // Then
            let get = build_message::<SurveyRef>(contract_account_id.clone())
                .call(|survey| survey.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), 0));

            Ok(())
        }
    }
}
