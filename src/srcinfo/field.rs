/// Field of a `.SRCINFO` file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Field<Name, Architecture> {
    name: Name,
    architecture: Option<Architecture>,
}

impl Field<(), ()> {
    /// Create a blank [`Field`].
    ///
    /// This function when combined with [`with_name`](Field::with_name) and [`with_architecture`](Field::with_architecture)
    /// would produce a usable [`Field`].
    pub fn blank() -> Self {
        Field {
            name: (),
            architecture: None,
        }
    }
}

impl<Name, Architecture> Field<Name, Architecture> {
    /// Replace the name of the field.
    pub fn with_name<NewName>(self, name: NewName) -> Field<NewName, Architecture> {
        Field {
            name,
            architecture: self.architecture,
        }
    }

    /// Replace the architecture suffix of the field.
    pub fn with_architecture<NewArchitecture>(
        self,
        architecture: Option<NewArchitecture>,
    ) -> Field<Name, NewArchitecture> {
        Field {
            name: self.name,
            architecture,
        }
    }

    /// Get an immutable reference to the name of the field.
    pub const fn name(&self) -> &'_ Name {
        &self.name
    }

    /// Get an immutable reference to the architecture suffix of the field.
    pub const fn architecture(&self) -> Option<&'_ Architecture> {
        self.architecture.as_ref()
    }

    /// Destructure into a tuple of field name and architecture.
    pub fn into_components(self) -> (Name, Option<Architecture>) {
        (self.name, self.architecture)
    }
}

impl<'a, Architecture> Field<&'a str, Architecture> {
    /// Get the name of the field as a string slice.
    pub const fn name_str(&self) -> &'_ str {
        self.name()
    }
}

impl<'a, Name> Field<Name, &'a str> {
    /// Get the name of the field as a string slice.
    pub fn architecture_str(&self) -> Option<&'_ str> {
        self.architecture().copied()
    }
}

/// Raw string field of a `.SRCINFO` file.
pub type RawField<'a> = Field<&'a str, &'a str>;

/// Parsed field of a `.SRCINFO` file.
pub type ParsedField<Architecture> = Field<FieldName, Architecture>;
impl<Architecture> ParsedField<Architecture> {
    /// Get the name of the field as a string slice.
    pub fn name_str(&self) -> &'static str {
        self.name().into()
    }
}

/// Convert a [`FieldName`] into a [`AnyField`] without an architecture.
impl<Architecture> From<FieldName> for ParsedField<Architecture> {
    fn from(field_name: FieldName) -> Self {
        Field::blank().with_name(field_name).with_architecture(None)
    }
}

mod name;
mod parse;

pub use name::*;
pub use parse::*;
