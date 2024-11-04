use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FlowControl {
    None = 0,
    XOnXOff = 1,
    RtsCts = 2,
    //RequestToSend = 2,
    //RequestToSendXOnXOff = 3,
}

impl FlowControl {
    pub fn as_serial_value(&self) -> serial2::FlowControl {
        match self {
            FlowControl::None => serial2::FlowControl::None,
            FlowControl::XOnXOff => serial2::FlowControl::XonXoff,
            FlowControl::RtsCts => serial2::FlowControl::RtsCts,
            //FlowControl::RequestToSend => serial2::FlowControl::RequestToSend,
            //FlowControl::RequestToSendXOnXOff => serial2::FlowControl::RequestToSendXOnXOff,
        }
    }
}

impl Default for FlowControl {
    fn default() -> Self {
        FlowControl::None
    }
}

impl<'a> Deserialize<'a> for FlowControl {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(FlowControl::None),
            1 => Ok(FlowControl::XOnXOff),
            2 => Ok(FlowControl::RtsCts),
            //3 => Ok(FlowControl::RequestToSendXOnXOff),
            _ => Err(serde::de::Error::custom(format!("invalid flow control value: {}", value))),
        }
    }
}

