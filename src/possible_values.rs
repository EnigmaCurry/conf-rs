use crate::traits::ValueEnum;
use clap::builder::PossibleValuesParser;

/// A single possible value for an option.
/// newtype of clap::builder::PossibleValue
#[derive(Debug)]
pub struct PossibleValue {
    /// The name will be used to decide whether this value was provided by the user to an argument.
    pub name: &'static str,
    /// Sets a *hidden* alias for this argument value.
    pub aliases: &'static [&'static str],
    /// Sets the help description of the value.
    pub help: Option<&'static str>,
}

/// get clap::builder::PossibleValuesParser from ValueEnum
pub fn possible_values_parser<T: ValueEnum>() -> PossibleValuesParser {
    let vals = T::possible_values()
        .iter()
        .map(|v| {
            let mut pv = clap::builder::PossibleValue::new(v.name);
            for &a in v.aliases {
                pv = pv.alias(a);
            }
            if let Some(h) = v.help {
                pv = pv.help(h);
            }
            pv
        })
        .collect::<Vec<_>>();

    PossibleValuesParser::new(vals)
}

pub fn possible_values_parser_from_slice(
    values: &'static [crate::PossibleValue],
) -> PossibleValuesParser {
    let vals = values
        .iter()
        .map(|v| {
            let mut pv = clap::builder::PossibleValue::new(v.name);
            for &a in v.aliases {
                pv = pv.alias(a);
            }
            if let Some(h) = v.help {
                pv = pv.help(h);
            }
            pv
        })
        .collect::<Vec<_>>();

    PossibleValuesParser::new(vals)
}
