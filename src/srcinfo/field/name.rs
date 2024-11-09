use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Designated position of a [`FieldName`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldNamePosition {
    /// The field name is `pkgbase`.
    BaseHeader,
    /// The field name is `pkgname`.
    DerivativeHeader,
    /// The field name is only valid under `pkgbase`.
    BaseSectionOnly,
    /// The field name is valid under either `pkgbase` or `pkgname`.
    AnySection,
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
        pub enum FieldName {
            $( #[strum(serialize = $header_name)] $header_variant, )*
            $( #[strum(serialize = $base_name)] $base_variant, )*
            $( #[strum(serialize = $shared_name)] $shared_variant, )*
        }

        impl FieldNamePosition {
            /// Determine a position of a field name.
            pub const fn for_name(field_name: FieldName) -> Self {
                match field_name {
                    FieldName::Base => FieldNamePosition::BaseHeader,
                    FieldName::Name => FieldNamePosition::DerivativeHeader,
                    $(FieldName::$base_variant => FieldNamePosition::BaseSectionOnly,)*
                    $(FieldName::$shared_variant => FieldNamePosition::AnySection,)*
                }
            }
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
