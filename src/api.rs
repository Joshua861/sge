use std::any::Any;

use crate::user_storage::get_user_storage;

pub fn storage_store_state<T: Any>(state: T) {
    get_user_storage().store(state);
}

pub fn storage_get_state<T: Any>() -> &'static T {
    get_user_storage().get()
}

pub fn storage_try_get_state<T: Any>() -> Option<&'static T> {
    get_user_storage().try_get()
}

pub fn storage_get_state_mut<T: Any>() -> &'static mut T {
    get_user_storage().get_mut()
}

pub fn storage_try_get_state_mut<T: Any>() -> Option<&'static mut T> {
    get_user_storage().try_get_mut()
}
