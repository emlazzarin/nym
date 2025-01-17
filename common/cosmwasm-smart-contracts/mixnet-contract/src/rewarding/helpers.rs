// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use cosmwasm_std::{Coin, Decimal, Uint128};

/// Truncates all decimal points so that the reward would fit in a `Coin` and so that we would
/// never attempt to reward more than the owner is due
/// for example it truncates "23.9" into "23"
pub fn truncate_reward(reward: Decimal, denom: impl Into<String>) -> Coin {
    let amount = truncate_reward_amount(reward);
    Coin {
        denom: denom.into(),
        amount,
    }
}

pub fn truncate_reward_amount(reward: Decimal) -> Uint128 {
    truncate_decimal(reward)
}

pub fn truncate_decimal(amount: Decimal) -> Uint128 {
    amount * Uint128::new(1)
}
