use na::Point3;
use nom::character::complete::{char, crlf, space0};
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
                let (rest, _) = preceded(take_until("\r\n"), tag("\r\n"))(rest)?;
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

pub fn msi_model_start(input: &str) -> IResult<&str, &str> {
    let model_head = tag("(1 Model\r\n");
    preceded(take_until("(1 Model\r\n"), model_head)(input)
}
pub fn parse_xyz(input: &str) -> IResult<&str, [f64; 3]> {
    let (rest, res) = terminated(
        tuple((
            terminated(alt((float, decimal)), space0),
            terminated(alt((float, decimal)), space0),
            alt((float, decimal)),
        )),
        tag("))\r\n"),
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
pub fn lattice_vector(input: &str) -> IResult<&str, [[f64; 3]; 3]> {
    let (rest, _) = preceded(take_until("A D A3 ("), tag("A D A3 ("))(input)?;
    let (rest, vector_a) = parse_xyz(rest)?;
    let (rest, _) = preceded(take_until("A D B3 ("), tag("A D B3 ("))(rest)?;
    let (rest, vector_b) = parse_xyz(rest)?;
    let (rest, _) = preceded(take_until("A D C3 ("), tag("A D C3 ("))(rest)?;
    let (rest, vector_c) = parse_xyz(rest)?;
    Ok((rest, [vector_a, vector_b, vector_c]))
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    let (rest, _) = tuple((tag("  ("), decimal, tag(" Atom\r\n")))(input)?;
    let (rest, element_line) = delimited(
        tag("    (A C ACL \""),
        separated_pair(decimal, char(' '), alpha1),
        tag("\")\r\n"),
    )(rest)?;
    let (element_id, element_name) = element_line;
    let (rest, xyz) = alt((
        preceded(
            tuple((
                tag("    (A C Label \""),
                alpha1,
                tag("\")\r\n"),
                tag("    (A D XYZ ("),
            )),
            parse_xyz,
        ),
        preceded(tag("    (A D XYZ ("), parse_xyz),
    ))(rest)?;
    let (rest, atom_id) = preceded(tag("    (A I Id "), decimal)(rest)?;
    let (rest, _) = tag(")\r\n  )\r\n")(rest)?;
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

    let test_flaw = std::fs::read_to_string("C2H4_flawed.msi").unwrap();
    // let (rest, _) = msi_model_start(&test_flaw).unwrap();
    // let (rest, atom) = many1(parse_atom)(rest).unwrap();
    let ad = MsiModel::try_from(test_flaw.as_str());
    match ad {
        Ok(ad) => println!("{:?}", ad),
        Err(e) => println!("{}", e),
    }
    let test_lat = read_to_string("SAC_GDY_V.msi").unwrap();
    let lat = MsiModel::try_from(test_lat.as_str()).unwrap();
    println!("{:?}", lat);
}
