use core::{
    fmt::{Debug, Display},
    str::FromStr,
};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

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
        pub enum BaseFieldName {
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }

        /// Field name of the `pkgname` section of a `.SRCINFO` file.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
        #[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
        #[strum(use_phf)]
        pub enum DerivativeFieldName {
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }

        def_cast! {
            (AnyFieldName HeaderFieldName) {$($header_variant)*}
            (AnyFieldName BaseFieldName) {$($base_variant)* $($shared_variant)*}
            (AnyFieldName DerivativeFieldName) {$($shared_variant)*}
            (BaseFieldName DerivativeFieldName) {$($shared_variant)*}
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

pub(crate) mod sealed {
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
impl sealed::Sealed for BaseFieldName {}
impl FieldName for BaseFieldName {}
impl sealed::Sealed for DerivativeFieldName {}
impl FieldName for DerivativeFieldName {}
