use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;

#[no_mangle]
pub extern "C" fn increment() {
    match runtime::get_key("counter_value") {
        Some(counter_value_key) => {
            let key = counter_value_key.try_into().unwrap_or_revert();
            let counter_value: u32 = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            storage::write(key, counter_value + 1);
        }
        None => {
            let key = storage::new_uref(1u32).into();
            runtime::put_key("counter_value", key);
        }
    };
}
