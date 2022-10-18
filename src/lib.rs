use std::ops::RangeInclusive;

use anchor_lang::{prelude::*, InstructionData};
use num_traits::{AsPrimitive, PrimInt};

#[event]
pub struct RequestVrf {
    pub ix_sighash: [u8; 8],
    pub ix_data: Vec<u8>,
    pub accounts: Vec<AccountMetaRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub struct AccountMetaRef {
    pub pubkey: Pubkey,
    pub is_writable: bool,
}

impl AccountMetaRef {
    pub fn mutable(mut self) -> Self {
        self.is_writable = true;
        self
    }
}

pub fn account_meta(pubkey: &impl anchor_lang::Key) -> AccountMetaRef {
    AccountMetaRef {
        pubkey: pubkey.key(),
        is_writable: false,
    }
}

pub fn request_random<T: InstructionData>(ix: T, accounts: Vec<AccountMetaRef>) {
    let data = ix.data();

    emit!(RequestVrf {
        ix_sighash: data[0..8].try_into().unwrap(),
        ix_data: data[8..].to_vec(),
        accounts
    });
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Random([u8; Random::BYTES]);

impl Default for Random {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Random {
    pub const BYTES: usize = 16;

    pub fn bound<T>(self, range: RangeInclusive<T>) -> T
    where
        T: PrimInt + AsPrimitive<i128>,
        i128: AsPrimitive<T>,
    {
        let v = i128::from_be_bytes(self.0);
        let bound: i128 = (*range.end() - *range.start()).as_();
        ((v % bound) + range.start().as_()).as_()
    }
}

impl From<[u8; Random::BYTES]> for Random {
    fn from(v: [u8; Random::BYTES]) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(AnchorSerialize)]
    struct A {
        random: Random,
        value: u32,
    }

    #[test]
    fn test() {
        let a = A {
            random: Random::default(),
            value: 1234,
        };

        let mut v = Vec::new();
        a.serialize(&mut v).unwrap();
        println!("{:?}\n{:?}", &v[0..32], &v[32..]);
    }

    #[test]
    fn r1() {
        let r = Random([111, 118, 107, 173, 240, 168, 69, 73, 10, 9, 142, 105, 124, 62, 45, 22]);
        println!("{}", r.bound(0u64..=u32::MAX as u64));
    }
}
