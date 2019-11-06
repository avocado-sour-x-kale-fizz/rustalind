use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", revc(&input));
        }
        Err(error) => println!("err {}", error)
    }
}

fn revc(dna_string: &str) -> String {
    let mut complement_string = String::new();
    for nucleotide in dna_string.chars().rev() {
        let complement = match nucleotide {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            anything_else => anything_else
        };
        complement_string.push(complement);
    }
    return complement_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "ACCGGGTTTT",
            revc("AAAACCCGGT")
        );
    }

    #[test]
    fn minimal() {
        assert_eq!("ACGT", revc("ACGT"));
    }

    #[test]
    fn preserves_non_nucleotides() {
        assert_eq!("ACGH", revc("HCGT"));
    }
}
