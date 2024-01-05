use core::fmt;
use base64::Engine;
use error_stack::{Context, Report, ResultExt};
use lazy_static::lazy_static;
use crate::unrailed_defs::{UnrailedGameDifficulty, UnrailedGameMode};

pub struct UnrailedSeed{
    pub val: u32,
    pub difficulty: UnrailedGameDifficulty,
    pub mode: UnrailedGameMode,
}

#[derive(Debug)]
pub struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.write_str("invalid argument")
    }
}
impl Context for InvalidArgumentError {}

lazy_static!{
    static ref BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
        &base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-")
            .expect("invalid alphabet"),
        base64::engine::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(true)
            .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent)
            .with_encode_padding(true)
    );
}

impl UnrailedSeed{

    pub fn from_str(seed: &str) -> error_stack::Result<Self, InvalidArgumentError> {
        //base64 decode
        let decoded = BASE64_ENGINE.decode(seed.as_bytes())
            .change_context(InvalidArgumentError)?;
        let val = u32::from_le_bytes(decoded[0..4].try_into().unwrap());
        let difficulty = match decoded[4] >> 5{
            0 => UnrailedGameDifficulty::Easy,
            1 => UnrailedGameDifficulty::Medium,
            2 => UnrailedGameDifficulty::Hard,
            3 => UnrailedGameDifficulty::Extreme,
            4 => UnrailedGameDifficulty::Kids,
            _ => return Err(Report::new(InvalidArgumentError).into()),
        };
        let mode = UnrailedGameMode::Time;
        Ok(Self{ val, difficulty, mode})
    }
}
