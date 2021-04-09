use crate::deployer;
use casper_contract::contract_api::runtime;

// Contract Description

#[no_mangle]
fn new_counter() {
    let counter_name: String = runtime::get_named_arg("name");
    deployer::deploy_counter_contract(&counter_name);
}
