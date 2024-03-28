use std::fmt::{Formatter, Debug};

use anchor_lang::prelude::*;
use fixed::types::U80F48;

#[account(zero_copy)]
pub struct Price {
    pub adaptor_token: Pubkey,
    pub price: WrappedU80F48,
    pub last_update: u64,
}

#[zero_copy]
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WrappedU80F48 {
    pub value: u128,
}

impl Debug for WrappedU80F48 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<U80F48> for WrappedU80F48 {
    fn from(value: U80F48) -> Self {
        Self {
            value: value.to_bits()
        }
    }
}

impl From<WrappedU80F48> for U80F48 {
    fn from(value: WrappedU80F48) -> Self {
        Self::from_bits(value.value)
    }
}