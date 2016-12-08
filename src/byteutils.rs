use std::ops::{Add, Sub, BitOr, BitXor, BitAnd};
use datatypes::{Byte, Word};

use num::{CheckedAdd, CheckedSub, PrimInt, ToPrimitive};


/// Return the "low" 8 bits of `val'
/// Note that `low' in this context is in a big-endian sense, e.g.
/// low8(0xBEEF) = 0xBE, which is the opposite of the way it is
/// represented in CpuState.
pub fn low8(val: Word) -> Byte {
    (val >> 8).to_u8().unwrap()
}

/// Return the big-endian "high" 8 bits of val
pub fn high8(val: Word) -> Byte {
    (val & 0xFF).to_u8().unwrap()
}

/// Join two Bytes into a Word
/// join8(0xBE, 0xEF) = 0xBEEF
pub fn join8(low: Byte, high: Byte) -> Word {
    let mut word = low.to_u16().unwrap();
    let high = high.to_u16().unwrap();
    word = word << 8;
    word = word + high;
    word
}

/// Replace the low byte of `val' with `low'
pub fn join_low8(val: Word, low: Byte) -> Word {
    join8(low, high8(val))
}

/// Replace the high byte of `val' with `high'
pub fn join_high8(val: Word, high: Byte) -> Word {
    join8(low8(val), high)
}

fn add_overflow(l_sign: bool, r_sign: bool, result_sign: bool) -> bool {
    (result_sign != l_sign) && (l_sign == r_sign)
}

fn sub_overflow(l_sign: bool, r_sign: bool, result_sign: bool) -> bool {
    ((!l_sign && r_sign) && result_sign) ||
    ((l_sign && !r_sign) && !result_sign)
}

fn and_or_overflow(_: bool, _: bool, _: bool) -> bool {
    false
}

#[inline]
fn checked_or<T: PrimInt>(left: &T, right: &T) -> Option<T> {
    Some(*left | *right)
}

#[inline]
fn checked_and<T: PrimInt>(left: &T, right: &T) -> Option<T> {
    Some(*left & *right)
}

/// Arithmetic functions. Functions generated from this macro take input
/// as Bytes or Words, calculate the output, and also compute the carry,
/// overflow, sign, and zero flags.
macro_rules! arithmetic (
    (
        $name:ident,
        $input_type:ident,
        $un_op:ident $ch_op:expr,
        $overflow_fn:ident
    ) => {
        pub fn $name(left: $input_type, right: $input_type)
        -> ($input_type, bool, bool, bool, bool) {
            let result = left.$un_op(right);

            let l_sign: bool = left.leading_zeros() == 0;
            let r_sign: bool = right.leading_zeros() == 0;
            let result_sign: bool = result.leading_zeros() == 0;

            let overflow: bool = $overflow_fn(l_sign, r_sign, result_sign);
            let zero: bool = result == 0;
            let carry: bool = match $ch_op(&left, &right) {
                Some(_) => false,
                None => true,
            };

            (result, carry, overflow, result_sign, zero)
        }
    }
);

arithmetic!(add8,  Byte, add CheckedAdd::checked_add, add_overflow);
arithmetic!(add16, Word, add CheckedAdd::checked_add, add_overflow);
arithmetic!(sub8,  Byte, sub CheckedSub::checked_sub, sub_overflow);
arithmetic!(sub16, Word, sub CheckedSub::checked_sub, sub_overflow);
arithmetic!(or8,   Byte, bitor checked_or,     and_or_overflow);
arithmetic!(or16,  Word, bitor checked_or,     and_or_overflow);
arithmetic!(xor16, Word, bitxor checked_or,    and_or_overflow);
arithmetic!(and16, Word, bitand checked_and,   and_or_overflow);
