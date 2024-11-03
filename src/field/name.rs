use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use subenum::subenum;

#[subenum(
    DbFieldName(
        doc = "Field name of a package's database descriptor.",
        derive(Debug, Clone, Copy, PartialEq, Eq), // core traits
        derive(AsRefStr, Display, EnumString, IntoStaticStr), // strum traits
        strum(use_phf),
    ),
    SrcInfoHeaderFieldName(
        doc = "Field name of a header of a `.SRCINFO` file.",
        derive(Debug, Clone, Copy, PartialEq, Eq), // core traits
        derive(AsRefStr, Display, EnumString, IntoStaticStr), // strum traits
        strum(use_phf),
    ),
    SrcInfoBaseFieldName(
        doc = "Field name that only appears in the `pkgbase` section of a `.SRCINFO` file.",
        derive(Debug, Clone, Copy, PartialEq, Eq), // core traits
        derive(AsRefStr, Display, EnumString, IntoStaticStr), // strum traits
        strum(use_phf),
    ),
)]
pub enum AnyFieldName {
    #[subenum(DbFieldName(strum(serialize = "FILENAME")))]
    FileName,
    #[subenum(
        DbFieldName(strum(serialize = "NAME")),
        SrcInfoHeaderFieldName(strum(serialize = "pkgname"))
    )]
    Name,
    #[subenum(
        DbFieldName(strum(serialize = "BASE")),
        SrcInfoHeaderFieldName(strum(serialize = "pkgbase"))
    )]
    Base,
    #[subenum(SrcInfoBaseFieldName(strum(serialize = "pkgver")))]
    UpstreamVersion,
    #[subenum(SrcInfoBaseFieldName(strum(serialize = "pkgrel")))]
    PackageRelease,
    #[subenum(SrcInfoBaseFieldName(strum(serialize = "epoch")))]
    Epoch,
    #[subenum(DbFieldName(strum(serialize = "VERSION")))]
    Version,
    #[subenum(DbFieldName(strum(serialize = "DESC")))]
    Description,
    #[subenum(DbFieldName(strum(serialize = "GROUPS")))]
    Groups,
    #[subenum(DbFieldName(strum(serialize = "CSIZE")))]
    CompressedSize,
    #[subenum(DbFieldName(strum(serialize = "ISIZE")))]
    InstalledSize,
    #[subenum(DbFieldName(strum(serialize = "MD5SUM")))]
    Md5Checksum,
    #[subenum(DbFieldName(strum(serialize = "SHA256SUM")))]
    Sha256Checksum,
    #[subenum(DbFieldName(strum(serialize = "PGPSIG")))]
    PgpSignature,
    #[subenum(DbFieldName(strum(serialize = "URL")))]
    Url,
    #[subenum(DbFieldName(strum(serialize = "LICENSE")))]
    License,
    #[subenum(DbFieldName(strum(serialize = "ARCH")))]
    Architecture,
    #[subenum(DbFieldName(strum(serialize = "BUILDDATE")))]
    BuildDate,
    #[subenum(DbFieldName(strum(serialize = "PACKAGER")))]
    Packager,
    #[subenum(DbFieldName(strum(serialize = "DEPENDS")))]
    Dependencies,
    #[subenum(DbFieldName(strum(serialize = "MAKEDEPENDS")))]
    MakeDependencies,
    #[subenum(DbFieldName(strum(serialize = "CHECKDEPENDS")))]
    CheckDependencies,
    #[subenum(DbFieldName(strum(serialize = "OPTDEPENDS")))]
    OptionalDependencies,
    #[subenum(DbFieldName(strum(serialize = "PROVIDES")))]
    Provides,
    #[subenum(DbFieldName(strum(serialize = "CONFLICTS")))]
    Conflicts,
    #[subenum(DbFieldName(strum(serialize = "REPLACES")))]
    Replaces,
}
