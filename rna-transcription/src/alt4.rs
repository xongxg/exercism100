use std::ptr::dangling;

#[derive(Debug, PartialEq, Eq)]
pub enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

impl TryFrom<char> for DnaNucleotide {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            c => Err(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

impl TryFrom<char> for RnaNucleotide {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'U' => Ok(Self::U),
            c => Err(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strands: Vec<DnaNucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Self {
            strands: dna
                .chars()
                .enumerate()
                .map(|(i, c)| c.try_into().map_err(|_| i))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strand: self
                .strands
                .iter()
                .map(|&node| match node {
                    DnaNucleotide::A => RnaNucleotide::U,
                    DnaNucleotide::C => RnaNucleotide::G,
                    DnaNucleotide::G => RnaNucleotide::C,
                    DnaNucleotide::T => RnaNucleotide::A,
                })
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: Vec<RnaNucleotide>,
}

impl Rna {
    fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Rna {
            strand: rna
                .chars()
                .enumerate()
                .map(|(i, c)| c.try_into().map_err(|_| i))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
