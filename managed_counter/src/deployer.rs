use casper_contract::contract_api::{runtime, storage};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints};

pub fn deploy_manager_contract(name: String) {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("new_counter"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("increment"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Groups(vec![]),
        EntryPointType::Contract,
    ));
    let (contract_hash, _) = storage::new_contract(entry_points, None, None, None);
    runtime::put_key(&name, contract_hash.into());
}

pub fn deploy_counter_contract(name: &str) {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("increment"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    let (contract_hash, _) = storage::new_contract(entry_points, None, None, None);
    runtime::put_key(&name, contract_hash.into());
    runtime::put_key(
        &format!("{}_hash", name),
        storage::new_uref(contract_hash).into(),
    );
}
