use std::borrow::Cow;

use crate::ValueType;

/// A zero-copy string parsed from an iCal input.
#[derive(Debug, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseString<'a>(Cow<'a, str>);

impl ParseString<'_> {
    pub fn to_owned(&self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(ref s) => ParseString(Cow::Owned(s.clone())),
        }
    }

    pub fn into_owned(self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(s) => ParseString(Cow::Owned(s)),
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> ParseString<'a> {
    pub fn unescape_by_value_type(self, value_type: ValueType) -> ParseString<'a> {
        match value_type {
            ValueType::Text => self.unescape_text(),
            _ => self,
        }
    }

    /// Reverses `Property::escape_text`.
    ///
    /// Single left-to-right pass so it is the exact inverse of the escaper:
    /// chained `String::replace` collapsed `\\` first, freeing the backslash
    /// to recombine with the next char and be eaten by a later rule.
    pub fn unescape_text(self) -> ParseString<'a> {
        if !self.0.contains('\\') {
            return self;
        }
        let input = self.0.as_ref();
        let mut out = String::with_capacity(input.len());
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some(',') => out.push(','),
                Some(';') => out.push(';'),
                // lenient: some producers escape the (unreserved) colon
                Some(':') => out.push(':'),
                Some('n') | Some('N') => out.push('\n'),
                // unknown escape or trailing backslash: keep verbatim
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        }
        out.into()
    }
}

impl PartialEq<Self> for ParseString<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl PartialEq<&str> for ParseString<'_> {
    fn eq(&self, rhs: &&str) -> bool {
        self.as_ref() == *rhs
    }
}

impl AsRef<str> for ParseString<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<ParseString<'static>> for String {
    fn from(val: ParseString<'static>) -> Self {
        val.0.into_owned()
    }
}

impl From<String> for ParseString<'static> {
    fn from(s: String) -> Self {
        ParseString(Cow::Owned(s))
    }
}

impl std::fmt::Display for ParseString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'i> From<&'i str> for ParseString<'i> {
    fn from(s: &'i str) -> Self {
        ParseString(Cow::Borrowed(s))
    }
}
