address 0x1 {

/// Module which defines keccak hashes for byte vectors.
///
module XHash {
    native public fun keccak_256(data: vector<u8>): vector<u8>;
}

}
