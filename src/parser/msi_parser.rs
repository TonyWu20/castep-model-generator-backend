use na::Point3;
use nom::character::complete::{char, line_ending, space0, space1};
use nom::combinator::recognize;
use nom::multi::many1;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::alpha1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::atom::Atom;

use super::{decimal, float};
extern crate nom;

#[derive(Debug)]
pub struct MsiModel {
    lattice_vectors: Option<[[f64; 3]; 3]>,
    atoms: Vec<Atom>,
}

impl MsiModel {
    pub fn new(lattice_vectors: Option<[[f64; 3]; 3]>, atoms: Vec<Atom>) -> Self {
        Self {
            lattice_vectors,
            atoms,
        }
    }

    pub fn lattice_vectors(&self) -> Option<[[f64; 3]; 3]> {
        self.lattice_vectors
    }

    pub fn atoms(&self) -> &[Atom] {
        self.atoms.as_ref()
    }
}

impl<'a> TryFrom<&'a str> for MsiModel {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (rest, _) = msi_model_start(value)?;
        let parse_lattice_vec = lattice_vector(rest);
        match parse_lattice_vec {
            Ok(res) => {
                let (rest, lattice_vectors) = res;
                let (rest, _) =
                    preceded(alt((take_until("\r\n"), take_until("\n"))), line_ending)(rest)?;
                let atoms_parsed = many1(parse_atom)(rest);
                match atoms_parsed {
                    Ok(atoms) => Ok(MsiModel::new(Some(lattice_vectors), atoms.1)),
                    Err(e) => Err(e),
                }
            }
            Err(_) => {
                let (_, atoms) = many1(parse_atom)(rest)?;
                Ok(MsiModel::new(None, atoms))
            }
        }
    }
}

/// Skip the header of the file.
fn msi_model_start(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(take_until("(1 Model\r\n"), tag("(1 Model\r\n")),
        preceded(take_until("(1 Model\n"), tag("(1 Model\n")),
    ))(input)
}

/// Parse XYZ in `msi` file. Since it possibly write `0` instead of `0.0`, we have to parse with `alt((float, decimal))`
fn parse_xyz(input: &str) -> IResult<&str, [f64; 3]> {
    let (rest, res) = terminated(
        tuple((
            terminated(alt((float, decimal)), space0),
            terminated(alt((float, decimal)), space0),
            alt((float, decimal)),
        )),
        alt((tag("))\r\n"), tag("))\n"))),
    )(input)?;
    let (x, y, z) = res;
    Ok((
        rest,
        [
            x.parse::<f64>().unwrap(),
            y.parse::<f64>().unwrap(),
            z.parse::<f64>().unwrap(),
        ],
    ))
}
/// Parse the lattice vector block in `msi` file format.
fn lattice_vector(input: &str) -> IResult<&str, [[f64; 3]; 3]> {
    let (rest, _) = preceded(take_until("A D A3 ("), tag("A D A3 ("))(input)?;
    let (rest, vector_a) = parse_xyz(rest)?;
    let (rest, _) = preceded(take_until("A D B3 ("), tag("A D B3 ("))(rest)?;
    let (rest, vector_b) = parse_xyz(rest)?;
    let (rest, _) = preceded(take_until("A D C3 ("), tag("A D C3 ("))(rest)?;
    let (rest, vector_c) = parse_xyz(rest)?;
    Ok((rest, [vector_a, vector_b, vector_c]))
}

/// Parse atom blocks in `msi` file format.
/// Use space1 to handle 2/4 spaces cases. Use `line_ending` to handle `\n` (in unix-format) or `\r\n` (in dos-format)
fn parse_atom<'b, 'a: 'b>(input: &'a str) -> IResult<&'a str, Atom> {
    // This gives the nth of `Atom` blocks.
    let (rest, _) = tuple((
        tuple((space1, tag("("))),
        decimal,
        tag(" Atom"),
        line_ending,
    ))(input)?;
    // Parser to recognize and consume `")\r\n` or `")\n`
    let quoted_ending_block = |input: &'a str| -> IResult<&'a str, &'b str> {
        recognize(tuple((tag("\")"), line_ending)))(input)
    };
    // Parser to recognize and consume `)\r\n` or `)\n`
    let ending_block = |input: &'a str| -> IResult<&'a str, &'b str> {
        recognize(tuple((tag(")"), line_ending)))(input)
    };
    // This will parse the atomic number and element name.
    let (rest, element_line) = delimited(
        tuple((space1, tag("(A C ACL \""))),
        separated_pair(decimal, char(' '), alpha1), // Example: (A C ACL "6 C")
        quoted_ending_block,
    )(rest)?;
    let (element_id, element_name) = element_line;
    // Alternative cases: with a line of `Label` before `XYZ`, or without
    let (rest, xyz) = alt((
        preceded(
            tuple((
                tuple((space1, tag("(A C Label \""))),
                alpha1,
                quoted_ending_block,
                tuple((space1, tag("(A D XYZ ("))),
            )),
            parse_xyz,
        ),
        preceded(tuple((space1, tag("(A D XYZ ("))), parse_xyz),
    ))(rest)?;
    // Parse `atom_id`
    let (rest, atom_id) = preceded(tuple((space1, tag("(A I Id "))), decimal)(rest)?;
    // Travel out the block
    let (rest, _) = tuple((ending_block, space1, ending_block))(rest)?;
    let element_id = element_id.parse::<u32>().unwrap();
    let atom_id = atom_id.parse::<u32>().unwrap();
    let xyz = Point3::from_slice(&xyz);
    Ok((
        rest,
        Atom::new(element_name.to_string(), element_id, xyz, atom_id),
    ))
}

#[cfg(test)]
#[test]
fn test_msi() {
    use std::fs::read_to_string;

    let test_flaw = read_to_string("COOH.msi").unwrap();
    // let (rest, _) = msi_model_start(&test_flaw).unwrap();
    // let (rest, atom) = many1(parse_atom)(rest).unwrap();
    let ad = MsiModel::try_from(test_flaw.as_str());
    match ad {
        Ok(ad) => println!("{:?}", ad),
        Err(e) => println!("{}", e),
    }
    let test_lat = read_to_string("SAC_GDY_Ag.msi").unwrap();
    let lat = MsiModel::try_from(test_lat.as_str()).unwrap();
    println!("{:?}", lat);
}
