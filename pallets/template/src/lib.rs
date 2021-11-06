#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

use sp_std::vec::Vec;

pub const MAX_DATA:u64 = 500;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as ESim {

		pub Balances get(fn get_balance): map hasher(blake2_128_concat) T::AccountId => u64;
		pub AdminAddress get(fn get_admin):T::AccountId;
		Init get(fn is_init):bool;

	}
}



decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		Initialized(AccountId),
		//Event emiited when user create the data 
		CreateData(AccountId, u64),

		//Event emitted when user send the data to other
        Transfer(AccountId, AccountId, u64), // (from, to, value)
	}
);

decl_error! {
	pub enum Error for Module<T: Config>  {
        
		/// Attempted to initialize the admin address after it had already been initialized
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn init(origin,to:AccountId) -> DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(!Self::is_init(),<Error<T>>::AlreadyInitialized);
			AdminAddress:put(&to);
			Init::put(true);
		    Ok(())
		}

		#[weigth = 10_000]
		fn crate(origin,value:u64) -> DispatchResult{
			let sender = ensure_signed(origin)?;
			<Balances<T>>::insert(sender,value);
			Ok(())
		}

		/// Transfer the data from one account to another
		#[weight = 10_000]
		fn transfer(_origin, to: T::AccountId, value: u64) -> DispatchResult {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance.checked_add(value).expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(RawEvent::Transfer(sender, to, value));
			Ok(())
		}

		#[weight = 10_000]
        fun check_balance(origin) -> DispatchResult{

			let caller = ensure_signed(origin)?;
			let balance = Self::get_balance(caller);
			Ok(())

		}
		#[weight = 10_000]
		fn get_balances() -> DispatchResult{


		}
		#[weight = 10_000]

		fun remove_balance(origin,to:AccountId) -> DispatchResult{
            
			let caller = ensure_signed(origin)?;
            let admin = Self::get_admin();
			<Balances<T>>::insert(&to,0);
			Ok(());
		}
	}
}
