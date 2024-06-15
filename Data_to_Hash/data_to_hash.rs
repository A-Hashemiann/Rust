use sha2::{Sha256, Digest};

fn main() {
   
    let data = "Hello, world";

    // create a new Sha256 instance
    let mut hasher = Sha256::new();

    // update the hasher with the data
    hasher.update(data);

    /
    let result = hasher.finalize();

    // print the resulting hash
    println!("SHA-256 Hash: {:x}", result);
}
