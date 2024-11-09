use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Field of a package description.
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

/// Raw string field of a package description.
pub type RawField<'a> = Field<&'a str, &'a str>;

/// Parsed field of a package description.
pub type ParsedField<Architecture> = Field<FieldName, Architecture>;

impl<Architecture> ParsedField<Architecture> {
    /// Get the name of the field as a string slice.
    pub fn name_str(&self) -> &'static str {
        self.name().into()
    }
}

/// Convert a [`FieldName`] into a [`ParsedField`] without an architecture.
impl<Architecture> From<FieldName> for ParsedField<Architecture> {
    fn from(value: FieldName) -> Self {
        Field::blank().with_name(value).with_architecture(None)
    }
}

macro_rules! def_downcast {
    ($(
        $(#[$attrs:meta])*
        $method:ident: $input:ident -> $output:ident {$($variant:ident)*}
    )*) => {$(
        impl $input {
            $(#[$attrs])*
            pub fn $method(self) -> Option<$output> {
                match self {
                    $($input::$variant => Some($output::$variant),)*
                    _ => None,
                }
            }
        }

        impl TryFrom<$input> for $output {
            type Error = $input;
            fn try_from(input: $input) -> Result<Self, Self::Error> {
                input.$method().ok_or(input)
            }
        }
    )*};
}

macro_rules! def_upcast {
    ($(
        $(#[$attrs:meta])*
        $method:ident: $input:ident -> $output:ident {$($variant:ident)*}
    )*) => {$(
        impl $input {
            $(#[$attrs])*
            pub fn $method(self) -> $output {
                match self {
                    $($input::$variant => $output::$variant,)*
                }
            }
        }

        impl From<$input> for $output {
            fn from(input: $input) -> Self {
                input.$method()
            }
        }
    )*};
}

macro_rules! def_field_name {
    (
        header {$(
            $header_variant:ident = $header_name:literal
        )*}
        base {$(
            $base_variant:ident = $base_name:literal
        )*}
        shared {$(
            $shared_variant:ident = $shared_name:literal
        )*}
    ) => {
        /// Field name of a package description.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum FieldName {
            $( #[strum(serialize = $header_name)] $header_variant, )*
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }
        def_downcast! {
            /// Convert a [`FieldName`] into a [`HeaderFieldName`].
            into_header: FieldName -> HeaderFieldName {$($header_variant)*}
            /// Convert a [`FieldName`] into a [`BaseOnlyFieldName`].
            into_base_only: FieldName -> BaseOnlyFieldName {$($base_variant)*}
            /// Convert a [`FieldName`] into a [`SectionFieldName`].
            into_section: FieldName -> SectionFieldName {$($base_variant)* $($shared_variant)*}
        }

        /// Header field name of a package description.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum HeaderFieldName {
            $( #[strum(serialize = $header_name)] $header_variant, )*
        }
        def_upcast! {
            /// Convert a [`HeaderFieldName`] into a [`FieldName`].
            into_field_name: HeaderFieldName -> FieldName {$($header_variant)*}
        }

        /// Field name of the `pkgbase` section a package description.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum BaseOnlyFieldName {
            $( #[strum(serialize = $base_name)] $base_variant, )*
        }
        def_upcast! {
            /// Convert a [`BaseOnlyFieldName`] into a [`FieldName`].
            into_field_name: BaseOnlyFieldName -> FieldName {$($base_variant)*}
            /// Convert a [`BaseOnlyFieldName`] into a [`SectionFieldName`].
            into_section: BaseOnlyFieldName -> SectionFieldName {$($base_variant)*}
        }

        /// Field name of any section a package description.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum SectionFieldName {
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }
        def_upcast! {
            /// Convert a [`SectionFieldName`] into a [`FieldName`].
            into_field_name: SectionFieldName -> FieldName {$($base_variant)* $($shared_variant)*}
        }
        def_downcast! {
            /// Convert a [`SectionFieldName`] into a [`BaseOnlyFieldName`].
            into_base_only: SectionFieldName -> BaseOnlyFieldName {$($base_variant)*}
        }
    };
}

def_field_name! {
    header {
        Base = "pkgbase"
        Name = "pkgname"
    }

    base {
        Epoch = "epoch"
        Release = "pkgrel"
        ValidPgpKeys = "validpgpkeys"
        Version = "pkgver"
    }

    shared {
        /* MISC */
        Architecture = "arch"
        Backup = "backup"
        ChangeLog = "changelog"
        Description = "pkgdesc"
        Groups = "groups"
        Install = "install"
        License = "license"
        NoExtract = "noextract"
        Options = "options"
        Source = "source"
        Url = "url"

        /* DEPENDENCIES */
        Dependencies = "depends"
        CheckDependencies = "checkdepends"
        MakeDependencies = "makedepends"
        OptionalDependencies = "optdepends"
        Provides = "provides"
        Conflicts = "conflicts"
        Replaces = "replaces"

        /* CHECKSUMS */
        Md5Checksums = "md5sums"
        Sha1Checksums = "sha1sums"
        Sha224Checksums = "sha224sums"
        Sha256Checksums = "sha256sums"
        Sha384Checksums = "sha384sums"
        Sha512Checksums = "sha512sums"
    }
}

mod parse;
pub use parse::*;
