//! Manage all game templates.
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	weights::Weight, 
	Parameter,
	dispatch::Vec,
    traits::Get
};
use sp_runtime::{DispatchError, traits::{AtLeast32Bit,Bounded}};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
mod default_weight;

pub trait WeightInfo {
	fn create_template() -> Weight;
}

#[derive(Encode, Decode, Default, Copy, Clone)]
pub struct Template <TemplateId, TemplateName> {
	template_id: TemplateId,
    template_name: TemplateName
}
pub trait Config: frame_system::Config{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type WeightInfo: WeightInfo;
	/// Only the account has the right to manage all game templates
    type OwnerAddress: Get<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as GameTemplateModule {
		pub Templates get(fn get_templates): Vec<Template<u32, u32>>;
		pub TemplateMap get(fn get_templatemap): map hasher(blake2_128_concat) u32 => Template<u32, u32>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		CreateTemplate(AccountId, u32, u32),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
        PermissionDenied,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error::<T>;
		fn deposit_event() = default;
		/// create new template
		#[weight = T::WeightInfo::create_template()]
		pub fn create_template(origin, template_name: u32) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);

            let mut templates = Self::get_templates();
            let new_template_id = templates.len() as u32;
            let newTemplate = Template{
                template_id: new_template_id,
                template_name: template_name,
            };
            templates.insert(templates.len(), newTemplate.clone());
            Templates::put(templates);
            TemplateMap::insert(new_template_id, newTemplate);
			// Send event notification
			RawEvent::CreateTemplate(sender, new_template_id, template_name);
			Ok(())
		}
	}
}