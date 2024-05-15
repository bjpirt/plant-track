use random_string::generate;

pub fn random_id(length: usize) -> String {
    let charset: &str = "abcdefghjklmnpqrstuvwxyz23456789";
    generate(length, charset)
}
