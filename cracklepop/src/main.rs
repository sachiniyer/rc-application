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

// Saw https://alicja.dev/blog/programming/rc/2017/01/31/return-programmer.html
// Seemed like a good quick test
fn recursive(i: Option<u8>) {
    let i = i.unwrap_or(1);
    if i > 100 {
        return;
    }
    println!(
        "{}",
        match (i % 3, i % 5) {
            (0, 0) => "CracklePop".to_string(),
            (0, _) => "Crackle".to_string(),
            (_, 0) => "Pop".to_string(),
            _ => i.to_string(),
        }
    );
    recursive(Some(i + 1));
}
