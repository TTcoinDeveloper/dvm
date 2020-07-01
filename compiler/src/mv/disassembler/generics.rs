use std::rc::Rc;
use std::collections::HashSet;
use libra::libra_vm::file_format::{
    StructFieldInformation, Kind, SignatureToken, StructHandleIndex, CompiledModuleMut, Signature,
};
use std::borrow::Cow;
use rand::prelude::*;
use crate::mv::disassembler::Encode;
use anyhow::Error;
use std::fmt::Write;

const GENERICS_PREFIX: [&str; 22] = [
    "T", "G", "V", "A", "B", "C", "D", "F", "H", "J", "K", "L", "M", "N", "P", "Q", "R", "S", "W",
    "X", "Y", "Z",
];

#[derive(Clone)]
pub struct Generics(Rc<GenericPrefix>);

pub enum GenericPrefix {
    SimplePrefix(&'static str),
    Generated(u16),
}

impl Generics {
    pub fn new(module: &CompiledModuleMut) -> Generics {
        let identifiers: HashSet<&str> = module.identifiers.iter().map(|i| i.as_str()).collect();

        let generic = if let Some(prefix) = GENERICS_PREFIX
            .iter()
            .find(|prefix| !identifiers.contains(*prefix))
        {
            GenericPrefix::SimplePrefix(*prefix)
        } else {
            GenericPrefix::Generated(rand::random())
        };

        Generics(Rc::new(generic))
    }

    pub fn create_generic(&self, index: usize, kind: Kind) -> Generic {
        Generic {
            prefix: self.clone(),
            index,
            kind,
        }
    }
}

#[derive(Clone)]
pub struct Generic {
    prefix: Generics,
    index: usize,
    kind: Kind,
}

impl Generic {
    pub fn as_name(&self) -> GenericName {
        GenericName(&self)
    }
}

impl Encode for Generics {
    fn encode<W: Write>(&self, w: &mut W, _indent: u8) -> Result<(), Error> {
        match self.0.as_ref() {
            GenericPrefix::SimplePrefix(p) => w.write_str(p)?,
            GenericPrefix::Generated(p) => write!(w, "TYPE_{}", p)?,
        }
        Ok(())
    }
}

impl Encode for Generic {
    fn encode<W: Write>(&self, w: &mut W, indent: u8) -> Result<(), Error> {
        self.prefix.encode(w, indent)?;

        if self.index != 0 {
            write!(w, "_{}", self.index)?;
        }

        match self.kind {
            Kind::All => { /* no-op */ }
            Kind::Resource => w.write_str(": resource")?,
            Kind::Copyable => w.write_str(": copyable")?,
        };
        Ok(())
    }
}

pub struct GenericName<'a>(&'a Generic);

impl<'a> Encode for GenericName<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: u8) -> Result<(), Error> {
        self.0.prefix.encode(w, indent)?;

        if self.0.index != 0 {
            write!(w, "_{}", self.0.index)?;
        }

        Ok(())
    }
}