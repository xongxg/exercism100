use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>(&'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna<'a>(Cow<'a, str>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(pos) = dna
            .chars()
            .position(|c| !matches!(c, 'C' | 'G' | 'T' | 'A'))
        {
            Err(pos)
        } else {
            Ok(Dna(dna))
        }
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
                _ => unreachable!("The DNA sequence was validated."),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(pos) = rna
            .chars()
            .position(|c| !matches!(c, 'G' | 'C' | 'T' | 'A'))
        {
            Err(pos)
        } else {
            Ok(Rna(Cow::Borrowed(rna)))
        }
    }
}
