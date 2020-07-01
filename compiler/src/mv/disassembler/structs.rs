use crate::mv::disassembler::Encode;
use anyhow::Error;
use std::fmt::Write;
use crate::mv::disassembler::generics::{Generics, Generic};
use libra::libra_vm::file_format::{
    StructFieldInformation, Kind, SignatureToken, StructHandleIndex, CompiledModuleMut, Signature,
    StructDefinition,
};
use crate::mv::disassembler::imports::{Imports, Import};

pub struct StructDef<'a> {
    is_nominal_resource: bool,
    is_native: bool,
    name: &'a str,
    type_params: Vec<Generic>,
    fields: Vec<Field<'a>>,
}

impl<'a> StructDef<'a> {
    pub fn new(
        def: &'a StructDefinition,
        module: &'a CompiledModuleMut,
        generic: &'a Generics,
        imports: &'a Imports<'a>,
    ) -> StructDef<'a> {
        let handler = &module.struct_handles[def.struct_handle.0 as usize];
        let name = module.identifiers[handler.name.0 as usize].as_str();

        let type_params = handler
            .type_parameters
            .iter()
            .enumerate()
            .map(|(i, k)| generic.create_generic(i, *k))
            .collect::<Vec<_>>();

        let fields = Self::extract_fields(module, &def.field_information, imports, &type_params);

       StructDef {
            is_nominal_resource: handler.is_nominal_resource,
            is_native: def.field_information == StructFieldInformation::Native,
            name,
            type_params,
            fields,
        }
    }

    fn extract_fields(
        module: &'a CompiledModuleMut,
        info: &'a StructFieldInformation,
        imports: &'a Imports,
        type_params: &[Generic],
    ) -> Vec<Field<'a>> {
        if let StructFieldInformation::Declared(fields) = info {
            fields
                .iter()
                .map(|def| Field {
                    name: module.identifiers[def.name.0 as usize].as_str(),
                    f_type: Self::extract_type_signature(
                        module,
                        &def.signature.0,
                        imports,
                        type_params,
                    ),
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn extract_type_signature(
        module: &'a CompiledModuleMut,
        signature: &'a SignatureToken,
        imports: &'a Imports,
        type_params: &[Generic],
    ) -> FType<'a> {
        match signature {
            SignatureToken::U8 => FType::Primitive("u8"),
            SignatureToken::Bool => FType::Primitive("bool"),
            SignatureToken::U64 => FType::Primitive("u64"),
            SignatureToken::U128 => FType::Primitive("u128"),
            SignatureToken::Address => FType::Primitive("address"),
            SignatureToken::Signer => FType::Primitive("signer"),

            SignatureToken::Vector(sign) => FType::Vec(Box::new(Self::extract_type_signature(
                module,
                sign.as_ref(),
                imports,
                type_params,
            ))),
            SignatureToken::Struct(struct_index) =>
                FType::Struct(Self::extract_struct_name(module, struct_index, imports)),
            SignatureToken::StructInstantiation(struct_index, typed) => {
                FType::StructInst(Self::extract_struct_name(module, struct_index, imports),
                                  typed
                                      .iter()
                                      .map(|t| Self::extract_type_signature(module, t, imports, type_params))
                                      .collect::<Vec<_>>(),
                )
            }
            SignatureToken::Reference(sign) => FType::Ref(Box::new(Self::extract_type_signature(
                module,
                sign.as_ref(),
                imports,
                type_params,
            ))),
            SignatureToken::MutableReference(sign) => FType::RefMut(Box::new(Self::extract_type_signature(
                module,
                sign.as_ref(),
                imports,
                type_params,
            ))),
            SignatureToken::TypeParameter(index) => {
                FType::Generic(type_params[*index as usize].clone())
            }
        }
    }

    fn extract_struct_name(
        module: &'a CompiledModuleMut,
        struct_index: &'a StructHandleIndex,
        imports: &'a Imports,
    ) -> FullStructName<'a> {
        let handler = &module.struct_handles[struct_index.0 as usize];

        let module_handler = &module.module_handles[handler.module.0 as usize];
        let module_name = module.identifiers[module_handler.name.0 as usize].as_str();
        let address = &module.address_identifiers[module_handler.address.0 as usize];
        let type_name = module.identifiers[handler.name.0 as usize].as_str();

        imports.get_import(address, module_name)
            .and_then(|import| Some(FullStructName { name: type_name, import: Some(import) }))
            .unwrap_or_else(|| FullStructName { name: type_name, import: None })
    }
}

impl<'a> Encode for StructDef<'a> {
    fn write<W: Write>(&self, w: &mut W, indent: u8) -> Result<(), Error> {
        let nominal_name = if self.is_nominal_resource {
            "resource struct"
        } else if self.is_native {
            "native struct"
        } else {
            "struct"
        };

        if self.is_native {
            writeln!(
                f,
                "{s:width$}{nominal_name} {name}{params};",
                s = "",
                width = indent,
                nominal_name = nominal_name,
                name = self.name,
                params = self.type_params,
            )
        } else {
            writeln!(
                f,
                "{s:width$}{nominal_name} {name}{params} {{\n{fields}{s:width$}}}",
                s = "",
                width = self.indent_size,
                nominal_name = nominal_name,
                name = self.name,
                params = self.type_params,
                fields = self.fields,
            )
        }
    }
}

pub struct Field<'a> {
    name: &'a str,
    f_type: FType<'a>,
}

pub enum FType<'a> {
    Generic(Generic),
    Primitive(&'static str),
    Ref(Box<FType<'a>>),
    RefMut(Box<FType<'a>>),
    Vec(Box<FType<'a>>),
    Struct(FullStructName<'a>),
    StructInst(FullStructName<'a>, Vec<FType<'a>>),
}

pub struct FullStructName<'a> {
    name: &'a str,
    import: Option<Import<'a>>,
}
