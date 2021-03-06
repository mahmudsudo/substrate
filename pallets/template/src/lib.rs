#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	
	#[pallet::storage]
	#[pallet::getter(fn something)]
	
	pub type Something<T> = StorageValue<_, List<T : Config + parity_scale_codec::Encode>>;

	#[derive(codec::Encode,scale_info::TypeInfo,Decode)]
	  pub  struct List<T:Config> {
		  AccountId : Vec<T>
	  }
	

	// Pallets use events to inform users when important changes are made.
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(10000)]
        pub fn add_user(origin: OriginFor<T>,new : T) -> DispatchResult {
            ensure_root(origin)?;
			//get the vector and the value
			// <Something<T>>::put(new);
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					//add new user
					old.AccountId.push(new);
					// Update the value in storage with the incremented result.
					
					Ok(())
				},
			}
            Ok(())
        }
		#[pallet::weight(0)]
        pub fn remove_user(origin: OriginFor<T>,user: T) -> DispatchResult {
            ensure_root(origin)?;
			//get the vector and the value
			// <Something<T>>::put(new);
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					//remove user
					old.remove(user);
					// Update the value in storage with the incremented result.
					
					Ok(())
				},
			}
            Ok(())
        }


		
	}
}
