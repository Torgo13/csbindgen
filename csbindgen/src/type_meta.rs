use crate::{alias_map::AliasMap, builder::BindgenOptions};

pub fn escape_name(str: &str) -> String {
    match str {
        // C# keywords: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/
        "abstract" | "as" | "base" | "bool" | "break" | "byte" | "case" | "catch" | "char"
        | "checked" | "class" | "const" | "continue" | "decimal" | "default" | "delegate"
        | "do" | "double" | "else" | "enum" | "event" | "explicit" | "extern" | "false"
        | "finally" | "fixed" | "float" | "for" | "foreach" | "goto" | "if" | "implicit" | "in"
        | "int" | "interface" | "internal" | "is" | "lock" | "long" | "namespace" | "new"
        | "null" | "object" | "operator" | "out" | "override" | "params" | "private"
        | "protected" | "public" | "readonly" | "ref" | "return" | "sbyte" | "sealed" | "short"
        | "sizeof" | "stackalloc" | "static" | "string" | "struct" | "switch" | "this"
        | "throw" | "true" | "try" | "typeof" | "uint" | "ulong" | "unchecked" | "unsafe"
        | "ushort" | "using" | "virtual" | "void" | "volatile" | "while" => "@".to_string() + str,
        x => x.to_string(),
    }
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub rust_type: RustType,
}

#[derive(Clone, Debug)]
pub struct FieldMember {
    pub name: String,
    pub rust_type: RustType,
}

#[derive(Clone, Debug)]
pub struct ExternMethod {
    pub method_name: String,
    pub doc_comment: Vec<String>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<RustType>,
}

impl ExternMethod {
    pub fn escape_doc_comment(&self) -> Option<String> {
        if self.doc_comment.is_empty() {
            return None;
        }

        let mut s = String::new();
        for (i, x) in self.doc_comment.iter().enumerate() {
            if i != 0 {
                s.push(' ');
            }
            let ss = x
                .trim_matches(&['=', ' ', '\"'] as &[_])
                .replace("\\n", "")
                .replace("<", "&lt;")
                .replace(">", "&gt;");
            s.push_str(ss.as_str());
        }

        Some(s)
    }
}

#[derive(Clone, Debug)]
pub struct RustType {
    pub type_name: String,
    pub type_kind: TypeKind,
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    Normal,
    Pointer(PointerType, Box<RustType>),
    FixedArray(String, Option<PointerType>),         // digits
    Function(Vec<Parameter>, Option<Box<RustType>>), // parameter, return
    Option(Box<RustType>),
}

#[derive(Clone, Debug)]
pub enum PointerType {
    ConstPointer,
    MutPointer,
    ConstPointerPointer,
    MutPointerPointer,
    ConstMutPointerPointer,
    MutConstPointerPointer,
}

#[derive(Clone, Debug)]
pub struct RustStruct {
    pub struct_name: String,
    pub fields: Vec<FieldMember>,
    pub is_union: bool,
}

#[derive(Clone, Debug)]
pub struct RustEnum {
    pub enum_name: String,
    pub fields: Vec<(String, Option<String>)>, // name, value
    pub repr: Option<String>,
}

impl RustType {
    pub fn to_rust_string(&self, type_path: &str) -> String {
        let mut sb = String::new();

        fn emit_pointer(sb: &mut String, p: &PointerType) {
            match p {
                ConstPointer => sb.push_str("*const"),
                MutPointer => sb.push_str("*mut"),
                ConstPointerPointer => sb.push_str("*const *const"),
                MutPointerPointer => sb.push_str("*mut *mut"),
                ConstMutPointerPointer => sb.push_str("*const *mut"),
                MutConstPointerPointer => sb.push_str("*mut *const"),
            };
        }

        let emit_type_name = |sb: &mut String| {
            if !(self.type_name.starts_with("c_")
                || self.type_name == "usize"
                || self.type_name == "isize"
                || (type_path.is_empty()))
            {
                sb.push_str(type_path);
                sb.push_str("::");
            }
            sb.push_str(self.type_name.as_str());
        };

        use PointerType::*;
        use TypeKind::*;
        match &self.type_kind {
            Normal => {
                emit_type_name(&mut sb);
            }
            Pointer(p, inner) => {
                emit_pointer(&mut sb, p);
                sb.push(' ');
                sb.push_str(inner.to_rust_string(type_path).as_str());
            }
            FixedArray(digits, pointer) => {
                if let Some(p) = pointer {
                    emit_pointer(&mut sb, p);
                    sb.push(' ');
                }

                sb.push('[');
                emit_type_name(&mut sb);
                sb.push_str("; ");
                sb.push_str(digits.as_str());
                sb.push(']');
            }
            Function(parameters, return_type) => {
                emit_type_name(&mut sb); // extern fn
                sb.push('(');
                let params = parameters
                    .iter()
                    .map(|x| {
                        format!(
                            "{}: {}",
                            escape_name(x.name.as_str()),
                            x.rust_type.to_rust_string(type_path)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                sb.push_str(params.as_str());
                sb.push(')');
                if let Some(t) = return_type {
                    sb.push_str(" -> ");
                    sb.push_str(t.to_rust_string(type_path).as_str());
                }
            }
            Option(inner) => {
                sb.push_str("Option<");
                sb.push_str(inner.to_rust_string(type_path).as_str());
                sb.push('>');
            }
        };

        sb
    }

    pub fn to_csharp_string(
        &self,
        options: &BindgenOptions,
        alias_map: &AliasMap,
        emit_from_struct: bool,
        method_name: &String,
        parameter_name: &String,
    ) -> String {
        fn convert_type_name(type_name: &str) -> String {
            let temp_string: String;
            let name = match type_name {
                // std::os::raw https://doc.rust-lang.org/std/os/raw/index.html
                // std::ffi::raw https://doc.rust-lang.org/core/ffi/index.html
                "c_char" => "byte",
                "c_schar" => "sbyte",
                "c_uchar" => "byte",
                "c_short" => "short",
                "c_ushort" => "ushort",
                "c_int" => "int",
                "c_uint" => "uint",
                "c_long" => "CLong",   // .NET 6
                "c_ulong" => "CULong", // .NET 6
                "c_longlong" => "long",
                "c_ulonglong" => "ulong",
                "c_float" => "float",
                "c_double" => "double",
                "c_void" => "void",
                "CString" => "sbyte",
                // rust primitives
                "i8" => "sbyte",
                "i16" => "short",
                "i32" => "int",
                "i64" => "long",
                "i128" => "Int128", // .NET 7
                "isize" => "nint",  // C# 9.0
                "u8" => "byte",
                "u16" => "ushort",
                "u32" => "uint",
                "u64" => "ulong",
                "u128" => "UInt128", // .NET 7
                "f32" => "float",
                "f64" => "double",
                "bool" => "bool",
                "char" => "uint",
                "usize" => "nuint", // C# 9.0
                "()" => "void",
                _ => {
                    temp_string = escape_name(type_name);
                    temp_string.as_str()
                }
            };
            name.to_string()
        }

        // resolve alias
        let (use_type, use_alias) = match alias_map.get_mapped_value(&self.type_name) {
            Some(x) => (x, true),
            None => (self.clone(), false),
        };

        // if alias if Option, unwrap.
        let type_csharp_string = if use_alias {
            use_type.to_csharp_string(
                options,
                alias_map,
                emit_from_struct,
                method_name,
                parameter_name,
            )
        } else {
            convert_type_name(use_type.type_name.as_str()).to_string()
        };

        let mut sb = String::new();

        match &self.type_kind {
            TypeKind::FixedArray(_, _) => {
                if emit_from_struct {
                    sb.push_str("fixed ");

                    let type_name = type_csharp_string.as_str();
                    let type_name = match type_name {
                    // C# fixed allow types
                    "bool" | "byte" | "short" | "int" | "long" | "char" | "sbyte" | "ushort"
                    | "uint" | "ulong" | "float" | "double" => type_name.to_owned(),
                    _ => format!("byte/* {}, this length is invalid so must keep pointer and can't edit from C# */", type_name)
                };

                    sb.push_str(type_name.as_str());
                } else {
                    let type_name = type_csharp_string.as_str();
                    sb.push_str(format!("void/* {}[] */", type_name).as_str());
                }
            }
            TypeKind::Function(parameters, return_type) => {
                if emit_from_struct && !options.csharp_use_function_pointer {
                    sb.push_str("void*");
                } else if options.csharp_use_function_pointer {
                    sb.push_str("delegate* unmanaged[Cdecl]");
                    sb.push('<');
                    for p in parameters {
                        sb.push_str(&p.rust_type.to_csharp_string(
                            options,
                            alias_map,
                            emit_from_struct,
                            method_name,
                            parameter_name,
                        ));
                        sb.push_str(", ");
                    }
                    match return_type {
                        Some(x) => {
                            sb.push_str(&x.to_csharp_string(
                                options,
                                alias_map,
                                emit_from_struct,
                                method_name,
                                parameter_name,
                            ));
                        }
                        None => {
                            sb.push_str("void");
                        }
                    };
                    sb.push('>');
                } else {
                    sb.push_str(build_method_delegate_name(method_name, parameter_name).as_str());
                }
            }
            TypeKind::Option(inner) => {
                // function pointer can not annotate ? so emit inner only
                sb.push_str(
                    inner
                        .to_csharp_string(
                            options,
                            alias_map,
                            emit_from_struct,
                            method_name,
                            parameter_name,
                        )
                        .as_str(),
                );
            }
            _ => {
                fn emit_pointer(
                    sb: &mut String,
                    rust_type: &RustType,
                    options: &BindgenOptions,
                    alias_map: &AliasMap,
                    emit_from_struct: bool,
                    method_name: &String,
                    parameter_name: &String,
                    emit_inner: bool,
                ) -> bool {
                    use PointerType::*;
                    if let TypeKind::Pointer(p, inner) = &rust_type.type_kind {
                        if emit_inner {
                            sb.push_str(
                                &inner
                                    .to_csharp_string(
                                        options,
                                        alias_map,
                                        emit_from_struct,
                                        method_name,
                                        parameter_name,
                                    )
                                    .as_str(),
                            );
                        }
                        match p {
                            MutPointer | ConstPointer => {
                                sb.push('*');
                            }
                            MutPointerPointer
                            | ConstPointerPointer
                            | MutConstPointerPointer
                            | ConstMutPointerPointer => {
                                sb.push_str("**");
                            }
                        }
                        return true;
                    }
                    false
                }

                let mut emit_inner = true;
                if use_alias {
                    if !emit_pointer(
                        &mut sb,
                        &use_type,
                        options,
                        alias_map,
                        emit_from_struct,
                        method_name,
                        parameter_name,
                        emit_inner,
                    ) {
                        sb.push_str(type_csharp_string.as_str());
                    }

                    emit_inner = false;
                }

                if !emit_pointer(
                    &mut sb,
                    &self,
                    options,
                    alias_map,
                    emit_from_struct,
                    method_name,
                    parameter_name,
                    emit_inner,
                ) {
                    if emit_inner {
                        sb.push_str(type_csharp_string.as_str());
                    }
                }
            }
        };

        sb
    }
}

pub fn build_method_delegate_if_required(
    me: &RustType,
    options: &BindgenOptions,
    alias_map: &AliasMap,
    method_name: &String,
    parameter_name: &String,
) -> Option<String> {
    let emit_from_struct = false;

    match &me.type_kind {
        TypeKind::Function(parameters, return_type) => {
            if emit_from_struct && !options.csharp_use_function_pointer {
                None
            } else if options.csharp_use_function_pointer {
                None
            } else {
                let return_type_name = match return_type {
                    Some(x) => x.to_csharp_string(
                        options,
                        alias_map,
                        emit_from_struct,
                        method_name,
                        parameter_name,
                    ),
                    None => "void".to_string(),
                };

                let joined_param = parameters
                    .iter()
                    .map(|p| {
                        let cs = p.rust_type.to_csharp_string(
                            options,
                            alias_map,
                            emit_from_struct,
                            method_name,
                            parameter_name,
                        );
                        format!("{} {}", cs, escape_name(p.name.as_str()))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let delegate_name = build_method_delegate_name(method_name, parameter_name);
                let delegate_code =
                    format!("delegate {return_type_name} {delegate_name}({joined_param})");
                Some(delegate_code)
            }
        }
        TypeKind::Option(inner) => build_method_delegate_if_required(
            inner,
            options,
            alias_map,
            method_name,
            parameter_name,
        ),
        _ => None,
    }
}

pub fn build_method_delegate_name(method_name: &String, parameter_name: &String) -> String {
    format!("{method_name}_{parameter_name}_delegate")
}

impl std::fmt::Display for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rust_string(""))
    }
}
