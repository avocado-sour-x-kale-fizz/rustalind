use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", rna(&input));
        }
        Err(error) => println!("err {}", error)
    }
}

fn rna(dna_string: &str) -> String {
    return dna_string.replace("T", "U");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            "GAUGGAACUUGACUACGUAAAUU",
            rna("GATGGAACTTGACTACGTAAATT")
        );
    }

    #[test]
    fn test_minimal() {
        assert_eq!("ACGU", rna("ACGT"));
    }
}
