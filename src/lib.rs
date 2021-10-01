use serde::Deserialize;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

// We try and parse each line of the input `value` as [Message] and group all
// messages by their `type_`. For each group, we gather their respective [GroupStats].
///
/// Returns an [InvalidMessageError] at the first [Message] it fails to parse.
///
///
/// NOTE FOR REVIEWERS
///
/// Given that each line, i.e., message, of the input file is an arbritrary
/// JSON object, from which we can only expect and only need the `type` field,
/// for simplicity and efficiency purposes we only parse said field in [Message].
///
/// Also for efficiency purposes, we build the returned groupped stats map
/// as we iterate through each message using a `mut HashMap` instead
/// of using an iterable and immutable approach such as first groupping every
/// message by their `type`, then parsing each message of each group, and finally
/// building each group's [GroupStats] by folding through each group's [Message]'s,
/// which would impose an O(n + n*m) vs the 0(n) we propose.
///
pub fn group_messages(value: String) -> Result<HashMap<String, GroupStats>, InvalidMessageError> {
    let mut groups: HashMap<String, GroupStats> = HashMap::new();
    for raw_message in value.lines() {
        let Message { type_ } = raw_message.try_into()?;
        let group = groups.entry(type_).or_default();
        *group += raw_message.into();
    }

    Ok(groups)
}

/// Captures a [serde_json::Error] when trying to deserializing a [Message].
#[derive(thiserror::Error, Debug)]
#[error("failed to deserialize Message from: '{0}'")]
pub struct InvalidMessageError(#[from] serde_json::Error);

/// A Message represents an arbirtrary JSON object of which
/// we only expect a `type` field to be present.
#[derive(Debug, Deserialize)]
pub struct Message {
    /// The type of a [Message]
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
}

impl TryFrom<&str> for Message {
    type Error = InvalidMessageError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(Into::into)
    }
}

/// The relevant stats of a group of [Message]s.
#[derive(Copy, Clone, Default)]
pub struct GroupStats {
    pub occurences: i32,
    pub total_byte_size: usize,
}

/// We build a [GroupStats] from a &str value assuming it to
/// contain a serialized [Message].
impl From<&str> for GroupStats {
    fn from(message: &str) -> Self {
        Self {
            occurences: 1,
            total_byte_size: message.bytes().len(),
        }
    }
}

impl std::ops::AddAssign for GroupStats {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            occurences: self.occurences + other.occurences,
            total_byte_size: self.total_byte_size + other.total_byte_size,
        }
    }
}
