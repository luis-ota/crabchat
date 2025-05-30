use rand::{Rng, distr::Alphanumeric};

pub async fn generate_room_code() -> String {
    let code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    code.to_uppercase()
}
