use derive_more::{AsRef, Deref};

/// Field of a package description.
#[derive(Debug, Clone, Copy, Eq, PartialEq)] // core traits
#[derive(AsRef, Deref, derive_more::Display)] // derive_more traits
#[display("%{_0}%")]
pub struct Field<Name>(Name);

impl<Name> Field<Name> {
    /// Get an immutable reference to the name of the field.
    pub const fn name(&self) -> &'_ Name {
        &self.0
    }

    /// Convert into the name of the field.
    pub fn into_name(self) -> Name {
        self.0
    }
}

/// Raw string field of a package description.
pub type RawField<'a> = Field<&'a str>;

impl<'a> RawField<'a> {
    /// Get the name of the field as a string slice.
    pub const fn name_str(&self) -> &'_ str {
        self.name()
    }
}

/// Parsed field of a package description.
pub type ParsedField = Field<DbFieldName>;

impl ParsedField {
    /// Create a new [`ParsedField`].
    pub const fn new(name: DbFieldName) -> Self {
        Field(name)
    }

    /// Get the name of the field as a string slice.
    pub fn name_str(&self) -> &'static str {
        self.name().into()
    }
}

impl From<DbFieldName> for ParsedField {
    fn from(value: DbFieldName) -> Self {
        ParsedField::new(value)
    }
}

mod name;
mod parse;

pub use name::DbFieldName;
pub use parse::{ParseFieldError, ParseRawFieldError};
