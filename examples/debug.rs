use std::fs::File;
use std::io::Read;

use bytecode_rs::ByteCode;

fn main() {
    let mut f = File::open("./examples/puts.mrb").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let mut mrb: ByteCode = buffer.into();

    let _header = mrb.take(20);
    dbg!(&mrb);
}
