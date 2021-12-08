// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::errors::PartialVMResult;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use tiny_keccak::{Hasher};
use smallvec::smallvec;
use std::collections::VecDeque;

pub fn native_keccak_256(
    context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let hash_arg = pop_arg!(arguments, Vec<u8>);

    let cost = native_gas(
        context.cost_table(),
        NativeCostIndex::KECCAK_256,
        hash_arg.len(),
    );

    let mut sha3 = ::tiny_keccak::Sha3::v256();
    let data = hash_arg.as_slice();
    sha3.update(&data);
    let mut output = [0u8; 32];
    sha3.finalize(&mut output);
    let hash_vec = output.to_vec();

   // let hash_vec = HashValue::sha3_256_of(hash_arg.as_slice()).to_vec();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(hash_vec)],
    ))
}
