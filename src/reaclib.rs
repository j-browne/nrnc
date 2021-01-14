#![allow(unused_variables, dead_code)]

use std::io::{BufRead, Read};
use error::{Res, ReaclibError};

pub mod error;

#[derive(Debug)]
pub struct Chapter(u8);

#[derive(Debug)]
pub struct Reaclib(Vec<Set>);

impl Chapter {
    pub fn new(chapter: u8) -> Res<Self> {
        match chapter {
            1 ..= 11 => Ok(Self(chapter)),
            c => Err(ReaclibError::InvalidChapter(c)),
        }
    }

    fn num_reactants(&self) -> u8 {
        match self.0 {
            1 ..= 3 | 11 => 1,
            4 ..= 7 => 2,
            8 ..= 9 => 3,
            10 => 4,
            _ => unreachable!("there was an invalid chapter number"),
        }
    }

    fn num_products(&self) -> u8 {
        match self.0 {
            1 | 4 | 8 => 1,
            2 | 5 | 9 | 10 => 2,
            3 | 6 => 3,
            7 | 11 => 4,
            _ => unreachable!("there was an invalid chapter number"),
        }
    }
}

#[derive(Debug)]
pub struct NucName(String);

#[derive(Debug)]
pub enum Resonance {
    NonResonant,
    Resonant,
    Weak,
}

#[derive(Debug)]
pub struct Set {
    reactants: Vec<NucName>,
    products: Vec<NucName>,
    label: String,
    resonance: Resonance,
    reverse: bool,
    q_value: f64,
    parameters: [f64; 7],
}

pub fn parse_reaclib_2<R: BufRead>(reader: &mut R) -> Res<Reaclib> {
    let mut buf = String::new();
    reader.read_line(&mut buf);
    let chap_num: u8 = buf.trim().parse()?;
    let chapter = Chapter::new(chap_num)?;
    let set = parse_set(reader, chapter);

    let sets = vec![set];
    Ok(Reaclib(sets))
}

fn parse_set<R: BufRead>(reader: &mut R, chapter: Chapter) -> Set {
    // FIXME: Read lines from BufRead and parse those?
    reader.consume(5);
    let reactants = (0..chapter.num_reactants()).map(|_| parse_nuc_name(reader)).collect();
    let products = (0..chapter.num_products()).map(|_| parse_nuc_name(reader)).collect();
    // There is space for 6 nuclides (which can be either reactant or products)
    // skip any that aren't used for this chapter.
    reader.consume(5 * (6 - (chapter.num_products() as usize + chapter.num_reactants() as usize)));
    reader.consume(8);

    let mut label = String::new();
    reader.take(4).read_to_string(&mut label);

    // FIXME: This is placeholder stuff
    let resonance = Resonance::NonResonant;
    let reverse = false;
    let q_value = 0.0;
    let parameters = [0.0; 7];
    Set{reactants, products, label, resonance, reverse, q_value, parameters}
}

fn parse_nuc_name<R: BufRead>(reader: &mut R) -> NucName {
    let mut buf = String::new();
    reader.take(5).read_to_string(&mut buf);
    NucName(buf)
}