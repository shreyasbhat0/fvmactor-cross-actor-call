use std::str::FromStr;

use fvm_shared::{address::Address, econ::TokenAmount};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    account: String,
    method: u64,
    params: String,
    value: u64,
}

/// Method num 2.
pub fn say_hello() -> Option<RawBytes> {
    let mut state = State::load();
    state.count += 1;
    state.save();

    let caller = sdk::message::caller();
    let origin = sdk::message::origin();
    let receiver = sdk::message::receiver();

    let ret = to_vec(
        format!(
            "Hello world {caller}/{origin}/{receiver} #{}!",
            &state.count
        )
        .as_str(),
    );
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

//method num 3
pub fn cross_actor_call(params: u32) -> Option<RawBytes> {
    sdk::vm::set_panic_handler();
    let message = sdk::message::params_raw(params).unwrap().1;
    let input: Input = serde_json::from_slice(&message).unwrap();

    let address = Address::from_str(&input.account);

    let address = match address {
        Ok(add) => add,
        Err(err) => panic!("{}", format!("{} {:?}", err, input)),
    };

    let call = sdk::send::send(
        &address,
        input.method,
        RawBytes::from(input.params.as_str().as_bytes().to_vec()),
        TokenAmount::from_atto(input.value),
    );

    match call {
        Ok(result) => Some(result.return_data),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
