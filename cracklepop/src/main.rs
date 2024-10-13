fn main() {
    (1..=100).for_each(|i| {
        println!(
            "{}",
            match (i % 3, i % 5) {
                (0, 0) => "CracklePop".to_string(),
                (0, _) => "Crackle".to_string(),
                (_, 0) => "Pop".to_string(),
                _ => i.to_string(),
            }
        )
    });
}
