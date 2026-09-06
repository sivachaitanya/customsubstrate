#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AddRemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub initial_validators: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            Validators::<T>::put(self.initial_validators.clone());
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
                validators.push(account.clone());
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
    fn new_session(_new_index: pallet_session::SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(pallet::Validators::<T>::get())
    }

    fn new_session_genesis(_new_index: pallet_session::SessionIndex) -> Option<Vec<T::AccountId>> {
        Some(pallet::Validators::<T>::get())
    }

    fn end_session(_end_index: pallet_session::SessionIndex) {}

    fn start_session(_start_index: pallet_session::SessionIndex) {}
}
