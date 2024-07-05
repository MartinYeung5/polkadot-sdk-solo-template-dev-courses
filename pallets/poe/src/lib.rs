#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet_prelude]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_support::Config{
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_support::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Prros<T: Config> = StorageMap<
        _,
        Blakr2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

    }

    #[pallet::error]
    pub enum Error<T> {

    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>)
    }
}