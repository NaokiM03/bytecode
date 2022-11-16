# bytecode

This library provides the ability to read bytecode.

## Usage

Add this to your `Cargo.toml`:
```toml
bytecode = "0.1.0"
```

and this to your source code:
```rust
use bytecode::ByteCode;
```

## Example

```rust
use bytecode::ByteCode;

fn main() {
    {
        let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);

        bytes += 3;

        let _first = bytes[0];
        let _second = bytes[1];

        let _subslice = &bytes[2..5];
    }

    {
        let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);

        match bytes.peek(3) {
            // omitted
            _ => {}
        }

        if bytes.starts_with("foo".as_bytes()) {
            // omitted
        }

        bytes.skip(2);

        let _subslice = bytes.take(4);
    }

    {
        let mut bytes = ByteCode::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00]);

        let _u16 = bytes.take_into_u16(); // u16::MAX
        let _u32 = bytes.take_into_u32(); // u32::MAX
    }
}
```

```rust
use std::fs::File;
use std::io::Read;

use bytecode::ByteCode;

fn main() {
    let mut f = File::open("./examples/puts.mrb").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let mut mrb: ByteCode = buffer.into();

    let _header = mrb.take(20);
    dbg!(&mrb);
}
```

![examples/debug.png](https://raw.githubusercontent.com/NaokiM03/bytecode-rs/ac37898d77e449aeea62ea1e07211278ecede600/examples/debug.png)

## License

`bytecode` is released under the MIT License.
