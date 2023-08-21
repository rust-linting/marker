#![feature(register_tool)]
#![register_tool(marker)]

fn main() {
    let mut something = Some(12);

    #[warn(marker::print_every_expr)]
    while let Some(_) = something {
        something = None;
    }
}
