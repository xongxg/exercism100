
///
///
///
/// https://exercism.org/tracks/rust/exercises/rna-transcription/solutions/themkat
///
macro_rules! validate_nucleotides {
    ($nucleotides:ident with valid entries $rules:expr) => {
        let ind = $nucleotides.as_bytes().iter().position(|v|!$rules.contains(v) );

        if let Some(i) = ind {
            Err(i)
        }else {
            Ok(Self{$nucleotides.to_string()})
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);
#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate_nucleotides!(dna with valid entries [b'A', b'C', b'G', b'T'])
    }

    pub fn into_rna(self) -> Rna {
        Rna(String::from_utf8(
            self.0
                .as_bytes()
                .iter()
                .map(|&nucleotide| match nucleotide {
                    b'G' => b'C',
                    b'C' => b'G',
                    b'T' => b'A',
                    b'A' => b'U',
                    _ => unreachable!("Dafuq"),
                })
                .collect(),
        )
        .unwrap())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate_nucleotides!(rna with valid entries [b'A', b'C', b'G', b'U'])
    }
}
