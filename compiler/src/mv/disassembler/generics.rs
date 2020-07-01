use std::rc::Rc;
use std::collections::HashSet;
use libra::libra_vm::file_format::{
    StructFieldInformation, Kind, SignatureToken, StructHandleIndex, CompiledModuleMut, Signature,
};
use std::borrow::Cow;
use rand::prelude::*;

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
