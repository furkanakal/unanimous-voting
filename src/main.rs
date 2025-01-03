use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;

fn main() {
    let config = ConfigBuilder::default().build();

    // Client-side: Generate keys
    let (client_key, server_key) = generate_keys(config);

    // Define clear votes (1 for yes, 0 for no)
    let clear_votes = vec![1u8, 1u8, 1u8, 0u8]; // Example votes

    // Encrypt the votes
    let encrypted_votes: Vec<FheUint8> = clear_votes
        .iter()
        .map(|&vote| FheUint8::encrypt(vote, &client_key))
        .collect();

    // Server-side: Set the server key for computation
    set_server_key(server_key);

    // Homomorphically sum the encrypted votes
    let mut encrypted_sum = encrypted_votes[0].clone();
    for vote in &encrypted_votes[1..] {
        encrypted_sum = &encrypted_sum + vote;
    }

    // Encrypt the number of voters
    let encrypted_num_voters = FheUint8::encrypt(clear_votes.len() as u8, &client_key);

    // Compare if the sum equals the number of voters
    let is_unanimous = encrypted_sum.eq(&encrypted_num_voters);

    // Client-side: Decrypt the result
    let unanimous_result: bool = is_unanimous.decrypt(&client_key);

    // Display the result
    if unanimous_result {
        println!("The vote is unanimous: True");
    } else {
        println!("The vote is unanimous: False");
    }
}
