use std::env::args;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: String) -> io::Result<()> {
    define_ast(
        &output_dir,
        &"Expr".to_string(),
        &vec![
            "Binary     : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping   : Box<Expr> expression".to_string(),
            "Literal    : Object value".to_string(),
            "Unary      : Token operator, Box<Expr> right".to_string(),
        ],
    )?;
    Ok(())
}

fn define_ast(out_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{}/{}.rs", out_dir, base_name.to_string());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    writeln!(file, "{}", "use crate::error::*;")?;
    writeln!(file, "{}", "use crate::token::*;")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        // Binary + Expr
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }
    writeln!(file, "\npub enum {base_name} {{")?;

    for t in &tree_types {
        writeln!(file, "    {}({})", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in &t.fields {
            writeln!(file, "    {},", f)?
        }
        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait ExprVisitor<T> {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}<T>(&self, expr: &{}) -> Result<T, CfgError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    writeln!(file, "}}\n")?;

    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(file, "    fn accept<T>(&self, visitor: dyn ExprVisitor) -> Result<T,CfgError> {{")?;
        writeln!(file, "        visitor.visit_{}_{}(self);", t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),)?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }

    Ok(())
}
