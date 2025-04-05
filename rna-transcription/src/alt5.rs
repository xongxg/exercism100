use std::fmt::Debug;

pub trait Nucleotide: TryFrom<char> + Debug + PartialEq + Eq {}

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

impl Nucleotide for DnaNucleotide {}

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

impl Nucleotide for RnaNucleotide {}

#[derive(Debug, PartialEq, Eq)]
pub struct Strand<Base: Nucleotide>(Vec<Base>);

pub type Dna = Strand<DnaNucleotide>;
pub type Rna = Strand<RnaNucleotide>;

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Self(
            dna.chars()
                .enumerate()
                .map(|(i, c)| c.try_into().map_err(|_| i))
                .collect::<Result<Vec<_>, usize>>()?,
        ))
    }

    pub fn into_rna(self) -> Rna {
        Strand(
            self.0
                .iter()
                .map(|&d| match d {
                    DnaNucleotide::A => RnaNucleotide::U,
                    DnaNucleotide::C => RnaNucleotide::G,
                    DnaNucleotide::G => RnaNucleotide::C,
                    DnaNucleotide::T => RnaNucleotide::A,
                })
                .collect(),
        )
    }
}
