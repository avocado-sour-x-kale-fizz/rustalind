use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", prot(&input));
        }
        Err(error) => println!("err {}", error),
    }
}

fn prot(mrna_string: &str) -> String {
    let mut protein = String::new();
    let mrna_nucleotides: Vec<char> = mrna_string.chars().collect();
    for amino_acid in mrna_nucleotides.chunks(3) {
        let codon = match amino_acid {
            &['G', 'C', 'A'] | &['G', 'C', 'C'] | &['G', 'C', 'G'] | &['G', 'C', 'U'] => 'A',
            &['U', 'G', 'C'] | &['U', 'G', 'U'] => 'C',
            &['G', 'A', 'C'] | &['G', 'A', 'U'] => 'D',
            &['G', 'A', 'A'] | &['G', 'A', 'G'] => 'E',
            &['U', 'U', 'C'] | &['U', 'U', 'U'] => 'F',
            &['G', 'G', 'A'] | &['G', 'G', 'C'] | &['G', 'G', 'G'] | &['G', 'G', 'U'] => 'G',
            &['C', 'A', 'C'] | &['C', 'A', 'U'] => 'H',
            &['A', 'U', 'A'] | &['A', 'U', 'C'] | &['A', 'U', 'U'] => 'I',
            &['A', 'A', 'A'] | &['A', 'A', 'G'] => 'K',
            &['C', 'U', 'A']
            | &['C', 'U', 'C']
            | &['C', 'U', 'G']
            | &['C', 'U', 'U']
            | &['U', 'U', 'A']
            | &['U', 'U', 'G'] => 'L',
            &['A', 'U', 'G'] => 'M',
            &['A', 'A', 'C'] | &['A', 'A', 'U'] => 'N',
            &['C', 'C', 'A'] | &['C', 'C', 'C'] | &['C', 'C', 'G'] | &['C', 'C', 'U'] => 'P',
            &['C', 'A', 'A'] | &['C', 'A', 'G'] => 'Q',
            &['A', 'G', 'A']
            | &['A', 'G', 'G']
            | &['C', 'G', 'A']
            | &['C', 'G', 'C']
            | &['C', 'G', 'G']
            | &['C', 'G', 'U'] => 'R',
            &['A', 'G', 'C']
            | &['A', 'G', 'U']
            | &['U', 'C', 'A']
            | &['U', 'C', 'C']
            | &['U', 'C', 'G']
            | &['U', 'C', 'U'] => 'S',
            &['U', 'A', 'A'] | &['U', 'A', 'G'] | &['U', 'G', 'A'] => break,
            &['A', 'C', 'A'] | &['A', 'C', 'C'] | &['A', 'C', 'G'] | &['A', 'C', 'U'] => 'T',
            &['G', 'U', 'A'] | &['G', 'U', 'C'] | &['G', 'U', 'G'] | &['G', 'U', 'U'] => 'V',
            &['U', 'G', 'G'] => 'W',
            &['U', 'A', 'C'] | &['U', 'A', 'U'] => 'Y',
            _ => continue,
        };
        protein.push(codon);
    }
    return protein;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "MAMAPRTEINSTRING",
            prot("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA")
        );
    }

    #[test]
    fn single_protein() {
        assert_eq!("M", prot("AUGUGA"));
    }

    #[test]
    fn skips_over_invalid_amino_acids() {
        assert_eq!("M", prot("AUGUGZZZZZZZZZZZZZZZ"));
    }

    #[test]
    fn whole_alphabet() {
        assert_eq!("AAAACCDDEEFFGGGGHHIIIKKLLLLLLMNNPPPPQQRRRRRRSSSSSSTTTTVVVVWYY", prot("GCAGCCGCGGCUUGCUGUGACGAUGAAGAGUUCUUUGGAGGCGGGGGUCACCAUAUAAUCAUUAAAAAGCUACUCCUGCUUUUAUUGAUGAACAAUCCACCCCCGCCUCAACAGAGAAGGCGACGCCGGCGUAGCAGUUCAUCCUCGUCUACAACCACGACUGUAGUCGUGGUUUGGUACUAU"));
    }
}
