use sputnikvm::*;
use sputnikvm_network_classic::*;
use bigint::*;
use std::rc::Rc;
use hexutil::read_hex;
use std::ops::DerefMut;

fn main() {
    let block = HeaderParams {
        beneficiary: Address::default(),
        timestamp: 0,
        number: U256::zero(),
        difficulty: U256::zero(),
        gas_limit: Gas::zero(),
    };
    let context = Context {
        address: Address::default(),
        caller: Address::default(),
        gas_limit: Gas::zero(),
        gas_price: Gas::zero(),
        value: U256::zero(),
        code: Rc::new(read_hex("0x7f9d8d4f41aa9c545820a32d2af89f32bd8f899e7322ef42ab4130a5e7c8e0e58f6055").unwrap()),
        data: Rc::new(read_hex("0x").unwrap()),
        origin: Address::default(),
        apprent_value: U256::zero(),
        is_system: false,
        is_static: false,
    };
    let mut vm= Box::new(SeqContextVM::<MainnetEIP160Patch>::new(context, block));
    handle_fire_without_rpc(vm.deref_mut());
    println!("VM returned: {:?}", vm.status());
    println!("VM out: {:?}", vm.out());
    for account in vm.accounts() {
        println!("{:?}", account);
    }
}

fn handle_fire_without_rpc(vm: &mut VM) {
    loop {
        match vm.fire() {
            Ok(()) => break,
            Err(RequireError::Account(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address)).unwrap();
            },
            Err(RequireError::AccountStorage(address, index)) => {
                vm.commit_account(AccountCommitment::Storage {
                    address,
                    index,
                    value: M256::zero(),
                }).unwrap();
            },
            Err(RequireError::AccountCode(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address)).unwrap();
            },
            Err(RequireError::Blockhash(number)) => {
                vm.commit_blockhash(number, H256::default()).unwrap();
            },
        }
    }
}
