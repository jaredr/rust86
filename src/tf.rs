use byteutils;
use operand::Flags;
use datatypes::{Byte, Word};


macro_rules! define_transform (
    (
        $name:ident,
        $size:ident,
        $arithmetic_fn:expr
    ) => {
        pub fn $name(left: $size, right: $size, _: Flags) -> ($size, Flags) {
            let (result, cf, of, sf, zf) = $arithmetic_fn(left, right);
            let flags = Flags {
                carry: cf,
                overflow: of,
                sign: sf,
                zero: zf,
            };
            (result, flags)
        }
    }
);

define_transform!(b_add, Byte, byteutils::b_add);
define_transform!(b_sub, Byte, byteutils::b_sub);
define_transform!(b_or, Byte, byteutils::b_or);
define_transform!(b_xor, Byte, byteutils::b_xor);
define_transform!(b_and, Byte, byteutils::b_and);

define_transform!(w_add, Word, byteutils::w_add);
define_transform!(w_sub, Word, byteutils::w_sub);
define_transform!(w_or, Word, byteutils::w_or);
define_transform!(w_xor, Word, byteutils::w_xor);
define_transform!(w_and, Word, byteutils::w_and);

pub fn b_noop(left: Byte, right: Byte, flags: Flags) -> (Byte, Flags) {
    (right, flags)
}

pub fn w_noop(left: Word, right: Word, flags: Flags) -> (Word, Flags) {
    (right, flags)
}

