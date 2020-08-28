extern crate crypto;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn main() {
    let mut hasher = Sha3::sha_256();
    hasher.input_str("hello");
    let result = hasher.result_str();
    println!("result = {}",result);
}
