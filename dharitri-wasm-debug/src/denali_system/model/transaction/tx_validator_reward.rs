use crate::denali_system::model::{AddressValue, BigUintValue};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::TxValidatorRewardRaw,
};

use super::tx_interpret_util::interpret_moax_value;

#[derive(Debug)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub moax_value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            moax_value: interpret_moax_value(from.value, from.moax_value, context),
        }
    }
}

impl IntoRaw<TxValidatorRewardRaw> for TxValidatorReward {
    fn into_raw(self) -> TxValidatorRewardRaw {
        TxValidatorRewardRaw {
            to: self.to.into_raw(),
            value: None,
            moax_value: Some(self.moax_value.into_raw()),
        }
    }
}
