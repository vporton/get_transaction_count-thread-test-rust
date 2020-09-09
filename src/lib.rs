extern crate primitive_types;
use std::thread;
use ethers::providers::{Provider, Http};
use futures::executor::block_on;
use ethers::types::NameOrAddress;

fn run_queries_body() {
    let address: primitive_types::H160 = primitive_types::H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let _cc = move |provider: Provider<Http>| {
        let gc_future =
            provider.get_transaction_count(NameOrAddress::Address(address), None);
        thread::spawn(|| { block_on(gc_future) });
    };
}
