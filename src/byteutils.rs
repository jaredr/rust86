use datatypes::{Byte, Word};


/**
 * Return the "low" 8 bits of `val', e.g. given 0xBEEF returns 0xBE
 */
pub fn low8(val: Word) -> Byte {
    (val >> 8)
}
    
/**
 * Return the "high" 8 bits of val, e.g. given 0xBEEF returns 0xEF
 */
pub fn high8(val: Word) -> Byte {
    (val & 0xFF)
}

/**
 * Join two Bytes into a Word
 */
pub fn join8(low: Byte, high: Byte) -> Word {
    let mut word: u16 = high;
    word = word << 8;
    word = word + low;
    word
}

/**
 * Replace the low byte of `val' with `low'
 */
pub fn join_low8(val: Word, low: Byte) -> Word {
    join8(high8(val), low)
}

/**
 * Replace the high byte of `val' with `high'
 */
pub fn join_high8(val: Word, high: Byte) -> Word {
    join8(high, low8(val))
}
