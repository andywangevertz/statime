#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSource {
    AtomicClock,
    Gnss,
    TerrestrialRadio,
    SerialTimeCode,
    Ptp,
    Ntp,
    HandSet,
    Other,
    InternalOscillator,
    ProfileSpecific(u8),
    Reserved,
    /// Time source is unknown. This is not an official variant from the spec, but we just need it in practise
    Unknown(u8),
}

impl TimeSource {
    pub fn to_primitive(&self) -> u8 {
        match self {
            Self::AtomicClock => 0x10,
            Self::Gnss => 0x20,
            Self::TerrestrialRadio => 0x30,
            Self::SerialTimeCode => 0x39,
            Self::Ptp => 0x40,
            Self::Ntp => 0x50,
            Self::HandSet => 0x60,
            Self::Other => 0x90,
            Self::InternalOscillator => 0xA0,
            Self::ProfileSpecific(p) => 0xF0 + *p,
            Self::Reserved => 0xFF,
            Self::Unknown(v) => *v,
        }
    }

    pub fn from_primitive(value: u8) -> Self {
        match value {
            0x10 => Self::AtomicClock,
            0x20 => Self::Gnss,
            0x30 => Self::TerrestrialRadio,
            0x39 => Self::SerialTimeCode,
            0x40 => Self::Ptp,
            0x50 => Self::Ntp,
            0x60 => Self::HandSet,
            0x90 => Self::Other,
            0xA0 => Self::InternalOscillator,
            0xF0..=0xFE => Self::ProfileSpecific(value - 0xF0),
            0xFF => TimeSource::Reserved,
            v => TimeSource::Unknown(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_protocol_values() {
        for i in 0..u8::MAX {
            let protocol = TimeSource::from_primitive(i);
            assert_eq!(protocol.to_primitive(), i);
        }

        assert_eq!(TimeSource::ProfileSpecific(5).to_primitive(), 0xF5);
    }
}
