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

define_transform!(add8, Byte, byteutils::add8);
define_transform!(sub8, Byte, byteutils::sub8);
define_transform!(or8,  Byte, byteutils::or8);

define_transform!(add16, Word, byteutils::add16);
define_transform!(sub16, Word, byteutils::sub16);
define_transform!(or16,  Word, byteutils::or16);
define_transform!(xor16, Word, byteutils::xor16);
define_transform!(and16, Word, byteutils::and16);

pub fn sbb16(left: Word, right: Word, flags: Flags) -> (Word, Flags) {
    let carry = match flags.carry {
        true => 1,
        false => 0,
    };
    sub16(left, right + carry, flags)
}

pub fn adc16(left: Word, right: Word, flags: Flags) -> (Word, Flags) {
    let carry = match flags.carry {
        true => 1,
        false => 0,
    };
    add16(left, right + carry, flags)
}

pub fn noop8(_: Byte, right: Byte, flags: Flags) -> (Byte, Flags) {
    (right, flags)
}

pub fn noop16(_: Word, right: Word, flags: Flags) -> (Word, Flags) {
    (right, flags)
}
