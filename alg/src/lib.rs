#![no_std]

use core::result::Result;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod test;

#[wasm_bindgen]
pub struct TmdsVal {
    /// Our first decision on the flowchart; whether ones or zeros 'dominate' the number.
    pub one_dominated: bool,
    pub repr: u16,
}

#[wasm_bindgen]
pub enum ParseError {
    Overflow,
    IllegalChar,
    UnrecognizedFormat,
}

#[wasm_bindgen]
pub fn parse_byte(input: &str) -> Result<u8, ParseError> {
    if let Some(input) = input.strip_prefix("0x") {
        let mut val: u8 = 0;
        for c in input.chars() {
            if val & 0xf0 != 0 {
                return Err(ParseError::Overflow);
            }

            val <<= 4;
            val += c.to_digit(16).ok_or(ParseError::IllegalChar)? as u8;
        }

        Ok(val)
    } else {
        Err(ParseError::UnrecognizedFormat)
    }
}

#[wasm_bindgen]
pub struct State {
    pub cnt: i64,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> Self {
        Self { cnt: 0, }
    }

    pub fn convert(&mut self, x: u8) -> TmdsVal {
        let one_dominated = x.count_ones() > 4 || (x.count_ones() == 4 && (x & 1 == 0));
        let q_m = encode_unbalanced(x, if one_dominated { xnor } else { xor });

        let q_m_byte = (q_m & 0xff) as u8;
        let q_m_8 = (q_m & (0b1 << 8)) >> 8;

        let repr = if self.cnt == 0 || q_m_byte.count_ones() == 4 {
            let mut out = (1 - q_m_8) << 9;
            out |= q_m_8 << 8;
            if q_m_8 == 1 {
                out |= q_m_byte as u16;
            } else {
                out |= !q_m_byte as u16;
            }

            if q_m_8 == 0 { // no way this doesn't all simplify!
                self.cnt += (q_m_byte.count_zeros() as i64) - (q_m_byte.count_ones() as i64);
            } else {
                self.cnt += (q_m_byte.count_ones() as i64) - (q_m_byte.count_zeros() as i64);
            }

            out
        } else {
            if (self.cnt > 0 && q_m_byte.count_ones() > q_m_byte.count_zeros()) || (self.cnt < 0 && q_m_byte.count_zeros() > q_m_byte.count_ones()) {
                let out = 1 << 9 | q_m_8 << 8 | (!q_m_byte as u16);
                self.cnt += 2 * (q_m_8 as i64) + (q_m_byte.count_zeros() as i64 - q_m_byte.count_ones() as i64);

                out
            } else {
                let out = 0 << 9 | q_m_8 << 8 | (q_m_byte as u16);
                self.cnt += 2 * ((1 - q_m_8) as i64) + (q_m_byte.count_ones() as i64 - q_m_byte.count_zeros() as i64);

                out
            }
        };

        TmdsVal { one_dominated, repr }
    }
}

fn encode_unbalanced(x: u8, f: impl Fn(bool, bool) -> bool) -> u16 {
    let mut r: u16 = (x & 0b1).into();

    for i in 0..7 {
        let r_component = ((r & (0b1 << i)) >> i) != 0;
        let x_component = ((x & (0b1 << i + 1)) >> i + 1) != 0;
        r |= (f(r_component, x_component) as u16) << i + 1;
    }

    // This'll give us XOR == 1 and XNOR == 0. (Recall XNOR is logical equivalence.)
    r |= (f(true, false) as u16) << 8;

    r
}

fn xor(lhs: bool, rhs: bool) -> bool {
    lhs ^ rhs
}

fn xnor(lhs: bool, rhs: bool) -> bool {
    !xor(lhs, rhs)
}
