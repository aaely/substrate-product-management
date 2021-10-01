#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
use pallet::*;

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
use uuid::Uuid;
	use crate::types::{Peptide, PeptideProfile, AminoAcid};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CreateRoleOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn get_product)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub (super) type Peptides<T: Config> = StorageMap<_, Twox64Concat, u128, (Peptide<T::AccountId, T::Moment>, PeptideProfile), ValueQuery>;

	#[pallet::storage]
	pub type IdByCount<T> = StorageMap<_, Twox64Concat, u32, u128, ValueQuery>;
	
	#[pallet::storage]
	pub type ProductCount<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_amino_acid)]
	pub (super) type AminoAcids<T: Config> = StorageMap<_, Twox64Concat, u128, AminoAcid, ValueQuery>;

	#[pallet::storage]
	pub type AminoAcidCount<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewProduct(u128, u32, T::AccountId, T::Moment),
		NewAminoAcid(u128, u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidProduct,
		/// Errors should have helpful documentation associated with them.
		InvalidAmino,
		InvalidProductProfile,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn new_peptide(origin: OriginFor<T>, name: Vec<u8>, price: u32, inventory: u32, chain: Vec<u128>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let _who = who.clone();
			let product_name = name.clone();
			let _uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, &product_name).as_u128();
			Peptides::<T>::insert(_uuid, (Peptide {
				id: _uuid,
				name,
				price,
				inventory,
				created_by: who,
				created_at: pallet_timestamp::Now::<T>::get(),
			}, PeptideProfile {
				product_ref: _uuid,
				chain,
				production_cost: 0,
				production_yield: 0
			}));

			let mut count = ProductCount::<T>::get();
			IdByCount::<T>::insert(count, _uuid);
			count += 1;

			// Emit an event.
			Self::deposit_event(Event::NewProduct(_uuid.clone(), inventory.clone(), _who, pallet_timestamp::Now::<T>::get()));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn new_amino(origin: OriginFor<T>, name: Vec<u8>, cost: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _who = who.clone();
			let _uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, &name).as_u128();
			AminoAcids::<T>::insert(_uuid, AminoAcid {
				id: _uuid,
				name,
				cost
			});
			Self::deposit_event(Event::NewAminoAcid(_uuid.clone(), cost.clone(), _who));
			Ok(())
		}
	}
}
