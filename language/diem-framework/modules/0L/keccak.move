address 0x1 {

/// Module which defines keccak hashes for byte vectors.
///
module OLCrypto {
    native public fun keccak_256(data: vector<u8>): vector<u8>;
}

}
