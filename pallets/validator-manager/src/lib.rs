#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use alloc::vec::Vec;
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AddRemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type MaxValidators: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, T::MaxValidators>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub initial_validators: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let validators =
                BoundedVec::<T::AccountId, T::MaxValidators>::try_from(self.initial_validators.clone())
                    .expect("initial_validators exceeds MaxValidators");
            Validators::<T>::put(validators);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ValidatorAdded { account: T::AccountId },
        ValidatorRemoved { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyValidator,
        TooManyValidators,
        NotValidator,
        EmptyValidatorSet,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_validator(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            T::AddRemoveOrigin::ensure_origin(origin)?;

            Validators::<T>::try_mutate(|validators| {
                ensure!(!validators.contains(&account), Error::<T>::AlreadyValidator);
                validators
                    .try_push(account.clone())
                    .map_err(|_| Error::<T>::TooManyValidators)?;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ValidatorAdded { account });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn remove_validator(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            T::AddRemoveOrigin::ensure_origin(origin)?;

            Validators::<T>::try_mutate(|validators| {
                let before = validators.len();
                validators.retain(|v| v != &account);
                ensure!(validators.len() != before, Error::<T>::NotValidator);
                ensure!(!validators.is_empty(), Error::<T>::EmptyValidatorSet);
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ValidatorRemoved { account });
            Ok(())
        }
    }
}

impl<T: pallet::Config> pallet_session::SessionManager<T::AccountId> for pallet::Pallet<T> {
    fn new_session(_new_index: sp_staking::SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(pallet::Validators::<T>::get().into_inner())
    }

    fn new_session_genesis(_new_index: sp_staking::SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(pallet::Validators::<T>::get().into_inner())
    }

    fn end_session(_end_index: sp_staking::SessionIndex) {}

    fn start_session(_start_index: sp_staking::SessionIndex) {}
}
