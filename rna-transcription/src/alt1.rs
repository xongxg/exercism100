///
/// https://exercism.org/tracks/rust/exercises/rna-transcription/solutions/heheshang
///
///
#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>(&'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .find_map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'T' => None,
                _ => Some(i),
            })
            .map(|i| Err(i))
            .unwrap_or_else(|| Ok(Dna(dna)))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("invalid char"),
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .find_map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'U' => None,
                _ => Some(i),
            })
            .map(|i| Err(i))
            .unwrap_or_else(Ok(Rna(rna.to_owned())))
    }
}
