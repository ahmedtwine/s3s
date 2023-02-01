use crate::default;
use crate::dto::RustTypes;
use crate::f;
use crate::gen::Codegen;
use crate::ops::Operations;
use crate::rust;
use crate::rust::default_value_literal;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Not;

pub fn codegen(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    g.lines([
        "use super::*;", //
        "",
        "use crate::dto::*;",
        "",
        "use std::io::Write;",
        "",
    ]);

    codegen_xml_ser(ops, rust_types, g);
    codegen_xml_de(ops, rust_types, g);
}

pub fn is_xml_payload(field: &rust::StructField) -> bool {
    let streaming = field.type_ == "StreamingBlob" || field.type_ == "SelectObjectContentEventStream";
    field.position == "payload" && field.type_ != "Policy" && streaming.not()
}

pub fn is_xml_output(ty: &rust::Struct) -> bool {
    ty.xml_name.is_some() || ty.fields.iter().any(|field| field.position == "xml")
}

fn codegen_xml_ser(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let mut root_type_names: BTreeSet<&str> = default();
    let mut field_type_names: BTreeSet<&str> = default();

    let mut q: VecDeque<&str> = default();

    for op in ops.values() {
        let ty_name = op.output.as_str();

        let rust_type = &rust_types[ty_name];
        let rust::Type::Struct(ty) = rust_type else { panic!() };

        if is_xml_output(ty) {
            root_type_names.insert(ty_name);
            field_type_names.insert(ty_name);
            q.push_back(ty_name);
        }

        let mut payload_count = 0;
        for field in &ty.fields {
            if is_xml_payload(field) {
                root_type_names.insert(&field.type_);
                field_type_names.insert(&field.type_);
                q.push_back(&field.type_);
                payload_count += 1;
            }
        }
        assert!(payload_count <= 1);
    }

    {
        let extra = ["Progress", "Stats"];
        for ty in extra {
            root_type_names.insert(ty);
            field_type_names.insert(ty);
            q.push_back(ty);
        }
    }

    while let Some(name) = q.pop_front() {
        let rust_type = &rust_types[name];
        match rust_type {
            rust::Type::Struct(ty) => {
                for field in &ty.fields {
                    let is_xml_field = field.position == "xml" || is_xml_payload(field);
                    if is_xml_field.not() {
                        continue;
                    }

                    let field_type = &rust_types[field.type_.as_str()];
                    if let rust::Type::List(list_ty) = field_type {
                        field_type_names.insert(list_ty.member.type_.as_str());
                        q.push_back(list_ty.member.type_.as_str());
                    } else {
                        field_type_names.insert(field.type_.as_str());
                        q.push_back(field.type_.as_str());
                    }
                }
            }
            rust::Type::Alias(_) => {}
            rust::Type::List(ty) => {
                field_type_names.insert(ty.member.type_.as_str());
                q.push_back(ty.member.type_.as_str());
            }
            rust::Type::StrEnum(ty) => {
                field_type_names.insert(ty.name.as_str());
            }
            rust::Type::StructEnum(ty) => {
                for variant in &ty.variants {
                    field_type_names.insert(variant.type_.as_str());
                    q.push_back(variant.type_.as_str());
                }
            }
            rust::Type::Provided(ty) => {
                assert!(matches!(ty.name.as_str(), "Body" | "Event"));
            }
            rust::Type::Map(_) => unimplemented!(),
            rust::Type::Timestamp(_) => {}
        }
    }

    for rust_type in field_type_names.iter().map(|&name| &rust_types[name]) {
        match rust_type {
            rust::Type::Struct(ty) => {
                g.ln(f!("impl SerializeContent for {} {{", ty.name));
                g.ln(f!(
                    "fn serialize_content<W: Write>(&self, {}: &mut Serializer<W>) -> SerResult {{",
                    if ty.fields.is_empty() { '_' } else { 's' }
                ));

                for field in ty.fields.iter().filter(|x| x.position == "xml") {
                    let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);

                    let field_ty = &rust_types[field.type_.as_str()];
                    if let rust::Type::List(list_ty) = field_ty {
                        if field.option_type {
                            g.ln(f!("if let Some(iter) = &self.{} {{", field.name));
                        } else {
                            g.ln("{");
                            g.ln(f!("let iter = &self.{};", field.name));
                        }
                        if field.xml_flattened {
                            g.ln(f!("s.flattened_list(\"{xml_name}\", iter)?;"));
                        } else {
                            let member_xml_name = list_ty.member.xml_name.as_deref().unwrap();
                            g.ln(f!("s.list(\"{xml_name}\", \"{member_xml_name}\", iter)?;"));
                        }
                        g.ln("}");
                    } else if let rust::Type::Timestamp(ts_ty) = field_ty {
                        let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");
                        if field.option_type {
                            g.ln(f!("if let Some(ref val) = self.{} {{", field.name));
                            g.ln(f!("s.timestamp(\"{xml_name}\", val, TimestampFormat::{fmt})?;"));
                            g.ln("}");
                        } else {
                            g.ln(f!("s.timestamp(\"{}\", &self.{}, TimestampFormat::{})?;", xml_name, field.name, fmt));
                        }
                    } else if field.option_type {
                        g.ln(f!("if let Some(ref val) = self.{} {{", field.name));
                        g.ln(f!("s.content(\"{xml_name}\", val)?;"));
                        g.ln("}");
                    } else {
                        g.ln(f!("s.content(\"{}\", &self.{})?;", xml_name, field.name));
                    }
                }

                g.ln("Ok(())");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::StrEnum(ty) => {
                g.ln(f!("impl SerializeContent for {} {{", ty.name));
                g.ln("fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {");

                g.ln("self.as_str().serialize_content(s)");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::StructEnum(ty) => {
                g.ln(f!("impl SerializeContent for {} {{", ty.name));
                g.ln("fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {");

                g.ln("match self {");

                for variant in &ty.variants {
                    g.ln(f!("Self::{0}(x) => s.content(\"{0}\", x),", variant.name));
                }

                g.ln("}");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::Alias(_) => {}
            rust::Type::Provided(_) => {}
            rust::Type::Timestamp(_) => {}
            rust::Type::List(_) => panic!(),
            rust::Type::Map(_) => panic!(),
        }
        g.lf();
    }

    for rust_type in root_type_names.iter().map(|&name| &rust_types[name]) {
        let rust::Type::Struct(ty) = rust_type else { panic!("{rust_type:#?}") };

        g.ln(f!("impl Serialize for {} {{", ty.name));
        g.ln("fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {");

        let xml_name = ty.xml_name.as_deref().unwrap_or(ty.name.as_str());
        g.ln(f!("s.content(\"{xml_name}\", self)"));

        g.ln("}");
        g.ln("}");

        g.lf();
    }
}

fn codegen_xml_de(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let mut root_type_names: BTreeMap<&str, Option<&str>> = default();
    let mut field_type_names: BTreeSet<&str> = default();

    let mut q: VecDeque<&str> = default();

    for op in ops.values() {
        let ty_name = op.input.as_str();

        let rust_type = &rust_types[ty_name];
        let rust::Type::Struct(ty) = rust_type else { panic!() };
        assert!(ty.xml_name.is_none());

        let mut payload_count = 0;
        for field in &ty.fields {
            if is_xml_payload(field) {
                root_type_names.insert(&field.type_, field.xml_name.as_deref());
                field_type_names.insert(&field.type_);
                q.push_back(&field.type_);
                payload_count += 1;
            }
        }
        assert!(payload_count <= 1);
    }

    {
        let extra = ["Progress", "Stats"];
        for ty in extra {
            root_type_names.insert(ty, None);
            field_type_names.insert(ty);
            q.push_back(ty);
        }
    }

    while let Some(name) = q.pop_front() {
        let rust_type = &rust_types[name];
        match rust_type {
            rust::Type::Struct(ty) => {
                for field in &ty.fields {
                    let is_xml_field = field.position == "xml" || is_xml_payload(field);
                    if is_xml_field.not() {
                        continue;
                    }

                    let field_type = &rust_types[field.type_.as_str()];

                    if let rust::Type::List(list_ty) = field_type {
                        field_type_names.insert(list_ty.member.type_.as_str());
                        q.push_back(list_ty.member.type_.as_str());
                    } else {
                        field_type_names.insert(field.type_.as_str());
                        q.push_back(field.type_.as_str());
                    }
                }
            }
            rust::Type::Alias(_) => {}
            rust::Type::List(ty) => {
                field_type_names.insert(ty.member.type_.as_str());
                q.push_back(ty.member.type_.as_str());
            }
            rust::Type::StrEnum(ty) => {
                field_type_names.insert(ty.name.as_str());
            }
            rust::Type::StructEnum(ty) => {
                for variant in &ty.variants {
                    field_type_names.insert(variant.type_.as_str());
                    q.push_back(variant.type_.as_str());
                }
            }
            rust::Type::Provided(ty) => {
                assert!(matches!(ty.name.as_str(), "Event"));
            }
            rust::Type::Map(_) => unimplemented!(),
            rust::Type::Timestamp(_) => {}
        }
    }

    for rust_type in field_type_names.iter().map(|&name| &rust_types[name]) {
        match rust_type {
            rust::Type::Struct(ty) => {
                g.ln(f!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name));
                g.ln(f!(
                    "fn deserialize_content({}: &mut Deserializer<'xml>) -> DeResult<Self> {{",
                    if ty.fields.is_empty() { "_" } else { "d" },
                ));

                for field in &ty.fields {
                    assert!(field.position == "xml");
                }

                for field in &ty.fields {
                    g.ln(f!("let mut {}: Option<{}> = None;", field.name, field.type_));
                }

                if ty.fields.is_empty().not() {
                    g.ln("d.for_each_element(|d, x| match x {");
                    for field in &ty.fields {
                        let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);
                        let field_name = field.name.as_str();
                        let field_type = &rust_types[field.type_.as_str()];

                        g.ln(f!("b\"{xml_name}\" => {{"));

                        if let rust::Type::List(list_ty) = field_type {
                            if field.xml_flattened {
                                g.ln(f!("let ans: {} = d.content()?;", list_ty.member.type_));
                                g.ln(f!("{field_name}.get_or_insert_with(List::new).push(ans);"));
                            } else {
                                g.ln(f!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}"));
                                g.ln(f!("{field_name} = Some(d.list_content(\"member\")?);"));
                            }
                        } else if let rust::Type::Timestamp(ts_ty) = field_type {
                            let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");

                            g.ln(f!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}"));

                            g.ln(f!("{field_name} = Some(d.timestamp(TimestampFormat::{fmt})?);"));
                        } else {
                            g.ln(f!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}"));
                            g.ln(f!("{field_name} = Some(d.content()?);"));
                        }

                        g.ln("Ok(())");
                        g.ln("}");
                    }
                    g.ln("_ => Err(DeError::UnexpectedTagName)");
                    g.ln("})?;");
                }

                g.ln("Ok(Self {");
                for field in &ty.fields {
                    if let Some(ref default_value) = field.default_value {
                        let literal = default_value_literal(default_value);
                        g.ln(f!("{0}: {0}.unwrap_or({1}),", field.name, literal));
                        continue;
                    }

                    if field.option_type {
                        g.ln(f!("{},", field.name));
                    } else {
                        g.ln(f!("{0}: {0}.ok_or(DeError::MissingField)?,", field.name));
                    }
                }
                g.ln("})");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::StrEnum(ty) => {
                g.ln(f!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name));
                g.ln("fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {");
                g.ln("String::deserialize_content(d).map(Self::from)");
                g.ln("}");
                g.ln("}");
            }
            rust::Type::StructEnum(ty) => {
                g.ln(f!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name));
                g.ln("fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {");

                g.ln("d.element(|d, x| match x {");
                for variant in &ty.variants {
                    g.ln(f!("b\"{0}\" => Ok(Self::{0}(d.content()?)),", variant.name));
                }
                g.ln("_ => Err(DeError::UnexpectedTagName)");
                g.ln("})");

                g.ln("}");
                g.ln("}");
            }

            rust::Type::Alias(_) => {}
            rust::Type::Provided(ty) => {
                assert!(matches!(ty.name.as_str(), "Event"));
            }
            rust::Type::List(_) => panic!(),
            rust::Type::Map(_) => panic!(),
            rust::Type::Timestamp(_) => {}
        }
        g.lf();
    }

    for (rust_type, xml_name) in root_type_names.iter().map(|(&name, xml_name)| (&rust_types[name], xml_name)) {
        let rust::Type::Struct(ty) = rust_type else { panic!("{rust_type:#?}") };

        g.ln(f!("impl<'xml> Deserialize<'xml> for {} {{", ty.name));
        g.ln("fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {");

        assert!(ty.xml_name.is_none()); // canary for <https://github.com/Nugine/s3s/issues/2>

        let xml_name = xml_name.or(ty.xml_name.as_deref()).unwrap_or(&ty.name);
        g.ln(f!("d.named_element(\"{xml_name}\", |d|d.content())"));

        g.ln("}");
        g.ln("}");
        g.lf();
    }
}
