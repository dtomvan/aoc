#![allow(unused, dead_code)]
use std::{fmt::Display, slice::SliceIndex};

use aoc_common::result::AocResult;
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = InputConverter(include_str!("../../inputs/day-16.txt")).convert()?;
    let bits = Bits::from(input);
    dbg!(bits);

    Err(anyhow::anyhow!("Not implemented."))
}

pub const BIT_AMOUNT: usize = 4;

pub struct InputConverter<'a>(&'a str);

impl Convert for InputConverter<'_> {
    type Target = Bytes;

    fn convert(self) -> Result<Self::Target, ConvertError> {
        self.0
            .trim()
            .chars()
            .map(|x| BinaryConverter(x).convert())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bit {
    One,
    Zero,
}

impl Bit {
    pub fn as_num(&self) -> u8 {
        match self {
            Bit::One => 1,
            Bit::Zero => 0,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_num())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bits(Vec<Bit>);

impl Bits {
    fn acc_num(input: &[&Bit]) -> usize {
        input.iter().rev().enumerate().fold(0, |acc, (i, x)| {
            acc + (x.as_num() as usize * 2_usize.pow(i as u32))
        })
    }

    fn get_num(&self, idx: impl SliceIndex<[Bit], Output = [Bit]>) -> usize {
        Self::acc_num(self[idx].iter().collect_vec().as_slice())
    }
}

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = String::with_capacity(self.0.len() + 2);
        acc.push_str("0b");
        for i in 0..self.0.len() {
            acc.push(self.0[i].as_num().to_string().pop().unwrap());
        }
        f.write_str(&acc)
    }
}

impl std::ops::Deref for Bits {
    type Target = Vec<Bit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Bytes> for Bits {
    fn from(b: Bytes) -> Self {
        Self(
            b.0.into_iter()
                .flat_map(|x| {
                    let mut bits = vec![Bit::Zero; BIT_AMOUNT];
                    for n in 0..=BIT_AMOUNT {
                        if *x & (1 << n) != 0 {
                            *bits.get_mut(n).unwrap() = Bit::One;
                        }
                    }
                    bits.reverse();
                    bits
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bytes(Vec<Byte>);

impl From<Vec<u8>> for Bytes {
    fn from(x: Vec<u8>) -> Self {
        Self(x.into_iter().map(Byte).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Byte(u8);

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}",
                ByteToCharConverter(self.clone())
                    .convert()
                    .map_err(|_| std::fmt::Error)?
            )
        } else {
            write!(f, "{:#06b}", self.0)
        }
    }
}

impl std::ops::Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ByteToCharConverter(Byte);

impl Convert for ByteToCharConverter {
    type Target = char;

    fn convert(self) -> Result<Self::Target, ConvertError> {
        match self.0 .0 {
            0 => Ok('0'),
            1 => Ok('1'),
            2 => Ok('2'),
            3 => Ok('3'),
            4 => Ok('4'),
            5 => Ok('5'),
            6 => Ok('6'),
            7 => Ok('7'),
            8 => Ok('8'),
            9 => Ok('9'),
            10 => Ok('A'),
            11 => Ok('B'),
            12 => Ok('C'),
            13 => Ok('D'),
            14 => Ok('E'),
            15 => Ok('F'),
            _ => Err(ConvertError::InvalidDigit),
        }
    }
}

impl FromIterator<u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter.into_iter().map(Byte)))
    }
}

pub struct BinaryConverter(char);

impl Convert for BinaryConverter {
    type Target = u8;

    fn convert(self) -> Result<Self::Target, ConvertError> {
        match self.0 {
            '0' => Ok(0b0000),
            '1' => Ok(0b0001),
            '2' => Ok(0b0010),
            '3' => Ok(0b0011),
            '4' => Ok(0b0100),
            '5' => Ok(0b0101),
            '6' => Ok(0b0110),
            '7' => Ok(0b0111),
            '8' => Ok(0b1000),
            '9' => Ok(0b1001),
            'A' => Ok(0b1010),
            'B' => Ok(0b1011),
            'C' => Ok(0b1100),
            'D' => Ok(0b1101),
            'E' => Ok(0b1110),
            'F' => Ok(0b1111),
            ' ' => Err(ConvertError::NoInput),
            _ => Err(ConvertError::InvalidDigit),
        }
    }
}

pub trait Parse: Sized {
    fn parse(input: Bits) -> (Self, Bits);
}

pub type PacketVersion = usize;

impl Parse for PacketVersion {
    fn parse(input: Bits) -> (Self, Bits) {
        (input.get_num(0..3), Bits(input[3..].to_vec()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralPacket {
    version: PacketVersion,
    value: usize,
}

impl Parse for LiteralPacket {
    fn parse(input: Bits) -> (Self, Bits) {
        let (version, input) = PacketVersion::parse(input);
        let mut valid_bits = Vec::new();
        let mut index = 0;
        let mut zero_seen = false;

        // Ignore the type id
        for mut next in input[3..].iter().chunks(5).into_iter() {
            let first = next.next();
            if first == Some(&Bit::Zero) {
                zero_seen = true;
            }
            for next in next.by_ref() {
                valid_bits.push(next);
            }
            index += 1;
            if zero_seen {
                break;
            }
        }

        let value = Bits::acc_num(&valid_bits);

        (Self { version, value }, Bits(input[index..].to_vec()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperatorPacket {
    version: PacketVersion,
    length: PacketLength,
    sub_packets: Vec<Packet>,
}

impl Parse for OperatorPacket {
    fn parse(input: Bits) -> (Self, Bits) {
        let (version, input) = PacketVersion::parse(input);
        let (length, mut input) = PacketLength::parse(input);
        match length {
            PacketLength::SubPackets(n) => {
                let mut sub_packets = Vec::with_capacity(n);
                for _ in 0..n {
                    let (packet, new_input) = Packet::parse(input);
                    input = new_input;
                    sub_packets.push(packet);
                }

                Self {
                    version,
                    length,
                    sub_packets,
                }
            }
            // TODO: make multi packet parser
            PacketLength::TotalLength(_) => todo!(),
        };
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketLength {
    SubPackets(usize),
    TotalLength(usize),
}

impl Parse for PacketLength {
    fn parse(input: Bits) -> (Self, Bits) {
        match input[0] {
            Bit::One => {
                let amount = Bits::acc_num(&input.iter().collect_vec()[1..12]);
                (Self::SubPackets(amount), Bits(input[12..].to_vec()))
            }
            Bit::Zero => {
                let amount = Bits::acc_num(&input.iter().collect_vec()[1..16]);
                (Self::TotalLength(amount), Bits(input[16..].to_vec()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Parse for Packet {
    fn parse(input: Bits) -> (Self, Bits) {
        let packet_type = &input[3..6];
        use Bit::*;
        match packet_type {
            &[One, Zero, Zero] => {
                let (packet, bits) = LiteralPacket::parse(input);
                (Self::Literal(packet), bits)
            }
            _ => unimplemented!(),
            // _ => Self::Operator(OperatorPacket::parse(input)),
        }
    }
}

pub trait Convert {
    type Target;
    fn convert(self) -> Result<Self::Target, ConvertError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConvertError {
    InvalidDigit,
    NoInput,
    Other(String),
}

impl std::error::Error for ConvertError {}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            ConvertError::InvalidDigit => "Invalid Digit",
            ConvertError::NoInput => "Invalid or no input",
            ConvertError::Other(s) => s,
        };
        write!(f, "A conversion error happened: {}", error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_bytes() {
        assert_eq!(
            InputConverter("0123456789ABCDEF").convert(),
            Ok(Bytes::from(vec![
                0b0000, 0b0001, 0b0010, 0b0011, 0b0100, 0b0101, 0b0110, 0b0111, 0b1000, 0b1001,
                0b1010, 0b1011, 0b1100, 0b1101, 0b1110, 0b1111
            ])),
        );
    }

    #[test]
    fn convert_bits() {
        use Bit::*;
        assert_eq!(
            Bits::from(InputConverter("0123456789ABCDEF").convert().unwrap()),
            Bits(vec![
                Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, Zero, Zero, One, Zero, Zero, Zero,
                One, One, Zero, One, Zero, Zero, Zero, One, Zero, One, Zero, One, One, Zero, Zero,
                One, One, One, One, Zero, Zero, Zero, One, Zero, Zero, One, One, Zero, One, Zero,
                One, Zero, One, One, One, One, Zero, Zero, One, One, Zero, One, One, One, One,
                Zero, One, One, One, One
            ]),
        );
    }
    #[test]
    fn display_impl() {
        // Bytes
        assert_eq!(&format!("{}", Byte(2)), "0b0010");
        assert_eq!(&format!("{}", Byte(15)), "0b1111");
        // Bit
        assert_eq!(&format!("{}", Bit::One), "1");
        assert_eq!(&format!("{}", Bit::Zero), "0");

        // Bits
        use Bit::*;
        assert_eq!(
            &format!("{}", Bits(vec![One, Zero, One, Zero, Zero])),
            "0b10100"
        );
    }

    #[test]
    fn get_bits() {
        use Bit::*;
        // 0b10100 or 20
        let bits = Bits(vec![One, Zero, One, Zero, Zero]);
        assert_eq!(bits.get_num(..), 20);
        // 0b100 or 4
        assert_eq!(bits.get_num(2..), 4);
    }

    #[test]
    fn parse_literal() {
        let bits = Bits::from(InputConverter("D2FE28").convert().unwrap());

        let (packet, _) = Packet::parse(bits);
        assert_eq!(
            packet,
            Packet::Literal(LiteralPacket {
                version: 6,
                value: 2021
            })
        );
    }
}
