//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

//! new-transaction
script{
use 0x1::XHash;
use 0x1::Vector;
fun main() {

  // length tests
  //
  // test1
  // short word
  let v1 = XHash::keccak_256(b"hello");
  let len1 = Vector::length(&v1);
  assert(len1 == 32, 1);

  // test2
  // long sentence
  v1 = XHash::keccak_256(b"this is a long sentence aaaaaaaaaa bbbbbbbb");
  len1 = Vector::length(&v1);
  assert(len1 == 32, 1);

  // test3
  // one byte
  v1 = XHash::keccak_256(b"h");
  len1 = Vector::length(&v1);
  assert(len1 == 32, 1);

  // simple strings
  // test1
  v1 = XHash::keccak_256(b"hello");
  let v2 = x"3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392";
  assert(v1 == v2, 1);

  // test2
  v1 = XHash::keccak_256(b"world");
  v2 = x"420baf620e3fcd9b3715b42b92506e9304d56e02d3a103499a3a292560cb66b2";
  assert(v1 == v2, 1);

}
}
