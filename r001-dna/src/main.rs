use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", dna(&input));
        }
        Err(error) => println!("err {}", error)
    }
}

fn dna(dna_string: &str) -> String {
    let nucleotide = ['A', 'C', 'G', 'T'];
    let mut results = vec![];
    for c in nucleotide.iter() {
        let counted = dna_string.chars().filter(|x| x == c).count();
        results.push(counted.to_string());
    }
    return results.join(" ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            "20 12 17 21",
            dna("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC")
        );
    }

    #[test]
    fn test_minimal() {
        assert_eq!("1 1 1 1", dna("ACGT"));
    }
}
