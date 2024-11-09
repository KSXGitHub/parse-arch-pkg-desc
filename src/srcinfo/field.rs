use core::{
    fmt::{Debug, Display},
    str::FromStr,
};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

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

macro_rules! def_alias {
    ($(
        $(#[$attrs:meta])*
        $alias:ident = $field_name:ty;
    )*) => {$(
        $(#[$attrs])*
        pub type $alias<Architecture> = Field<$field_name, Architecture>;

        impl<Architecture> $alias<Architecture> {
            /// Get the name of the field as a string slice.
            pub fn name_str(&self) -> &'static str {
                self.name().into()
            }
        }

        #[doc = concat!("Convert a [`", stringify!($field_name), "`] into a [`", stringify!($alias), "`] without an architecture.")]
        impl<Architecture> From<$field_name> for $alias<Architecture> {
            fn from(field_name: $field_name) -> Self {
                Field::blank().with_name(field_name).with_architecture(None)
            }
        }
    )*};
}

def_alias! {
    /// Parsed field of a `.SRCINFO` file.
    AnyField = AnyFieldName;
    /// Parsed field of a header of a `.SRCINFO` file.
    HeaderField = HeaderFieldName;
    /// Parsed field of the `pkgbase` section of a `.SRCINFO` file.
    BaseOnlyField = BaseOnlyFieldName;
    /// Parsed field of any section of a `.SRCINFO` file.
    SectionField = SectionFieldName;
}

macro_rules! def_cast {
    ($(($up:ident $down:ident) {$($variant:ident)*})*) => {$(
        impl From<$up> for Option<$down> {
            fn from(input: $up) -> Self {
                match input {
                    $($up::$variant => Some($down::$variant),)*
                    _ => None,
                }
            }
        }

        impl TryFrom<$up> for $down {
            type Error = $up;
            fn try_from(input: $up) -> Result<Self, Self::Error> {
                Option::<Self>::from(input).ok_or(input)
            }
        }

        impl From<$down> for $up {
            fn from(input: $down) -> Self {
                match input {
                    $($down::$variant => $up::$variant,)*
                }
            }
        }
    )*}
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
        /// Field name of a `.SRCINFO` file.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum AnyFieldName {
            $( #[strum(serialize = $header_name)] $header_variant, )*
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }

        /// Header field name of a `.SRCINFO` file.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum HeaderFieldName {
            $( #[strum(serialize = $header_name)] $header_variant, )*
        }

        /// Field name of the `pkgbase` section of a `.SRCINFO` file.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum BaseOnlyFieldName {
            $( #[strum(serialize = $base_name)] $base_variant, )*
        }

        /// Field name of any section of a `.SRCINFO` file.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum SectionFieldName {
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }

        def_cast! {
            (AnyFieldName HeaderFieldName) {$($header_variant)*}
            (AnyFieldName BaseOnlyFieldName) {$($base_variant)*}
            (AnyFieldName SectionFieldName) {$($base_variant)* $($shared_variant)*}
            (SectionFieldName BaseOnlyFieldName) {$($base_variant)*}
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

mod sealed {
    pub trait Sealed {}
}

/// Field name of a `.SRCINFO` file.
pub trait FieldName:
    sealed::Sealed
    + Debug
    + Display
    + Clone
    + Copy
    + PartialEq
    + Eq
    + AsRef<str>
    + FromStr
    + for<'r> TryFrom<&'r str>
    + Into<&'static str>
    + TryFrom<AnyFieldName>
    + Into<AnyFieldName>
{
    /// Convert the field name into an instance of [`AnyFieldName`]
    fn into_any(self) -> AnyFieldName {
        self.into()
    }
}

impl sealed::Sealed for AnyFieldName {}
impl FieldName for AnyFieldName {}
impl sealed::Sealed for BaseOnlyFieldName {}
impl FieldName for BaseOnlyFieldName {}
impl sealed::Sealed for SectionFieldName {}
impl FieldName for SectionFieldName {}

mod parse;
pub use parse::*;
