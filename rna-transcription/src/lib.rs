mod alt1;
mod alt2;
mod alt3;
mod alt4;
mod alt5;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a> {
    sequence: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl<'a> Dna<'a> {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(p) = dna.chars().position(|c| !Self::valid_dna_nucleotide(c)) {
            return Err(p);
        }

        Ok(Dna { sequence: dna })
    }

    fn valid_dna_nucleotide(c: char) -> bool {
        c == 'G' || c == 'C' || c == 'T' || c == 'A'
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            sequence: self
                .sequence
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => panic!("bad dna"),
                })
                .collect::<String>(),
        }
    }
}

impl<'a> Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(p) = rna.chars().position(|c| !Self::valid_rna_nucleotide(c)) {
            return Err(p);
        }

        Ok(Rna {
            sequence: rna.to_string(),
        })
    }

    fn valid_rna_nucleotide(c: char) -> bool {
        c == 'C' || c == 'G' || c == 'A' || c == 'U'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rna_sequence() {
        let input = "";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn rna_complement_of_cytosine_is_guanine() {
        let input = "C";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("G").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn rna_complement_of_guanine_is_cytosine() {
        let input = "G";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("C").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn rna_complement_of_thymine_is_adenine() {
        let input = "T";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("A").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn rna_complement_of_adenine_is_uracil() {
        let input = "A";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("U").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn rna_complement() {
        let input = "ACGTGGTCTTAA";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("UGCACCAGAAUU").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn invalid_dna_input() {
        let input = "U";
        let output = Dna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn invalid_dna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Dna::new(input);
        let expected = Err(4);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn invalid_rna_input() {
        let input = "T";
        let output = Rna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn invalid_rna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Rna::new(input);
        let expected = Err(3);
        assert_eq!(output, expected);
    }
}
