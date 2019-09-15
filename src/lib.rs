#![allow(non_upper_case_globals)]

use libc::*;
#[cfg(feature = "with-freetype")]
mod freetype;
#[cfg(feature = "with-freetype")]
pub use self::freetype::*;

pub type FcChar8 = c_uchar;
pub type FcChar16 = c_ushort;
pub type FcChar32 = c_uint;
pub type FcBool = c_int;

pub const FC_MAJOR: c_int = 2;
pub const FC_MINOR: c_int = 13;
pub const FC_REVISION: c_int = 1;
pub const FC_VERSION: c_int = FC_MAJOR * 10000 + FC_MINOR * 100 + FC_REVISION;

pub const FC_CACHE_VERSION_NUMBER: c_int = 7;
macro_rules! FC_CACHE_VERSION { () => (7) }

pub const FcFalse: FcBool = 0;
pub const FcTrue: FcBool = 1;
pub const FcDontCare: FcBool = 2;

pub const FC_FAMILY: &str = "family\x00";
pub const FC_STYLE: &str = "style\x00";
pub const FC_SLANT: &str = "slant\x00";
pub const FC_WEIGHT: &str = "weight\x00";
pub const FC_SIZE: &str = "size\x00";
pub const FC_ASPECT: &str = "aspect\x00";
pub const FC_PIXEL_SIZE: &str = "pixelsize\x00";
pub const FC_SPACING: &str = "spacing\x00";
pub const FC_FOUNDRY: &str = "foundry\x00";
pub const FC_ANTIALIAS: &str = "antialias\x00";
pub const FC_HINTING: &str = "hinting\x00";
pub const FC_HINT_STYLE: &str = "hintstyle\x00";
pub const FC_VERTICAL_LAYOUT: &str = "verticallayout\x00";
pub const FC_AUTOHINT: &str = "autohint\x00";
pub const FC_GLOBAL_ADVANCE: &str = "globaladvance\x00";
pub const FC_WIDTH: &str = "width\x00";
pub const FC_FILE: &str = "file\x00";
pub const FC_INDEX: &str = "index\x00";
pub const FC_FT_FACE: &str = "ftface\x00";
pub const FC_RASTERIZER: &str = "rasterizer\x00";
pub const FC_OUTLINE: &str = "outline\x00";
pub const FC_SCALABLE: &str = "scalable\x00";
pub const FC_COLOR: &str = "color\x00";
pub const FC_VARIABLE: &str = "variable\x00";
pub const FC_SCALE: &str = "scale\x00";
pub const FC_SYMBOL: &str = "symbol\x00";
pub const FC_DPI: &str = "dpi\x00";
pub const FC_RGBA: &str = "rgba\x00";
pub const FC_MINSPACE: &str = "minspace\x00";
pub const FC_SOURCE: &str = "source\x00";
pub const FC_CHARSET: &str = "charset\x00";
pub const FC_LANG: &str = "lang\x00";
pub const FC_FONTVERSION: &str = "fontversion\x00";
pub const FC_FULLNAME: &str = "fullname\x00";
pub const FC_FAMILYLANG: &str = "familylang\x00";
pub const FC_STYLELANG: &str = "stylelang\x00";
pub const FC_FULLNAMELANG: &str = "fullnamelang\x00";
pub const FC_CAPABILITY: &str = "capability\x00";
pub const FC_FONTFORMAT: &str = "fontformat\x00";
pub const FC_EMBOLDEN: &str = "embolden\x00";
pub const FC_EMBEDDED_BITMAP: &str = "embeddedbitmap\x00";
pub const FC_DECORATIVE: &str = "decorative\x00";
pub const FC_LCD_FILTER: &str = "lcdfilter\x00";
pub const FC_FONT_FEATURES: &str = "fontfeatures\x00";
pub const FC_FONT_VARIATIONS: &str = "fontvariations\x00";
pub const FC_NAMELANG: &str = "namelang\x00";
pub const FC_PRGNAME: &str = "prgname\x00";
pub const FC_HASH: &str = "hash\x00";
pub const FC_POSTSCRIPT_NAME: &str = "postscriptname\x00";

pub const FC_CACHE_SUFFIX: &str = concat!(".cache-", FC_CACHE_VERSION!(), "\x00");
pub const FC_DIR_CACHE_FILE: &str = concat!("fonts.cache-", FC_CACHE_VERSION!(), "\x00");
pub const FC_USER_CACHE_FILE: &str = concat!(".fonts.cache-", FC_CACHE_VERSION!(), "\x00");

pub const FC_CHARWIDTH: &str = "charwidth\x00";
pub const FC_CHAR_WIDTH: &str = FC_CHARWIDTH;
pub const FC_CHAR_HEIGHT: &str = "charheight\x00";
pub const FC_MATRIX: &str = "matrix\x00";

pub const FC_WEIGHT_THIN: c_int = 0;
pub const FC_WEIGHT_EXTRALIGHT: c_int = 40;
pub const FC_WEIGHT_ULTRALIGHT: c_int = FC_WEIGHT_EXTRALIGHT;
pub const FC_WEIGHT_LIGHT: c_int = 50;
pub const FC_WEIGHT_DEMILIGHT: c_int = 55;
pub const FC_WEIGHT_SEMILIGHT: c_int = FC_WEIGHT_DEMILIGHT;
pub const FC_WEIGHT_BOOK: c_int = 75;
pub const FC_WEIGHT_REGULAR: c_int = 80;
pub const FC_WEIGHT_NORMAL: c_int = FC_WEIGHT_REGULAR;
pub const FC_WEIGHT_MEDIUM: c_int = 100;
pub const FC_WEIGHT_DEMIBOLD: c_int = 180;
pub const FC_WEIGHT_SEMIBOLD: c_int = FC_WEIGHT_DEMIBOLD;
pub const FC_WEIGHT_BOLD: c_int = 200;
pub const FC_WEIGHT_EXTRABOLD: c_int = 205;
pub const FC_WEIGHT_ULTRABOLD: c_int = FC_WEIGHT_EXTRABOLD;
pub const FC_WEIGHT_BLACK: c_int = 210;
pub const FC_WEIGHT_HEAVY: c_int = FC_WEIGHT_BLACK;
pub const FC_WEIGHT_EXTRABLACK: c_int = 215;
pub const FC_WEIGHT_ULTRABLACK: c_int = FC_WEIGHT_EXTRABLACK;

pub const FC_SLANT_ROMAN: c_int = 0;
pub const FC_SLANT_ITALIC: c_int = 100;
pub const FC_SLANT_OBLIQUE: c_int = 110;

pub const FC_WIDTH_ULTRACONDENSED: c_int = 50;
pub const FC_WIDTH_EXTRACONDENSED: c_int = 63;
pub const FC_WIDTH_CONDENSED: c_int = 75;
pub const FC_WIDTH_SEMICONDENSED: c_int = 87;
pub const FC_WIDTH_NORMAL: c_int = 100;
pub const FC_WIDTH_SEMIEXPANDED: c_int = 113;
pub const FC_WIDTH_EXPANDED: c_int = 125;
pub const FC_WIDTH_EXTRAEXPANDED: c_int = 150;
pub const FC_WIDTH_ULTRAEXPANDED: c_int = 200;

pub const FC_PROPORTIONAL: c_int = 0;
pub const FC_DUAL: c_int = 90;
pub const FC_MONO: c_int = 100;
pub const FC_CHARCELL: c_int = 110;

pub const FC_RGBA_UNKNOWN: c_int = 0;
pub const FC_RGBA_RGB: c_int = 1;
pub const FC_RGBA_BGR: c_int = 2;
pub const FC_RGBA_VRGB: c_int = 3;
pub const FC_RGBA_VBGR: c_int = 4;
pub const FC_RGBA_NONE: c_int = 5;

pub const FC_HINT_NONE: c_int = 0;
pub const FC_HINT_SLIGHT: c_int = 1;
pub const FC_HINT_MEDIUM: c_int = 2;
pub const FC_HINT_FULL: c_int = 3;

pub const FC_LCD_NONE: c_int = 0;
pub const FC_LCD_DEFAULT: c_int = 1;
pub const FC_LCD_LIGHT: c_int = 2;
pub const FC_LCD_LEGACY: c_int = 3;

pub type FcType = c_int;
pub const FcTypeUnknown: FcType = -1;
pub const FcTypeVoid: FcType = 0;
pub const FcTypeInteger: FcType = 1;
pub const FcTypeDouble: FcType = 2;
pub const FcTypeString: FcType = 3;
pub const FcTypeBool: FcType = 4;
pub const FcTypeMatrix: FcType = 5;
pub const FcTypeCharSet: FcType = 6;
pub const FcTypeFTFace: FcType = 7;
pub const FcTypeLangSet: FcType = 8;
pub const FcTypeRange: FcType = 9;

#[repr(C)] #[derive(Debug, Clone, PartialEq)]
pub struct FcMatrix { pub xx: c_double, pub xy: c_double, pub yx: c_double, pub yy: c_double }
impl Default for FcMatrix
{
    /// FcMatrixInit
    fn default() -> Self
    {
        FcMatrix { xx: 1.0, yy: 1.0, xy: 0.0, yx: 0.0 }
    }
}

pub enum FcCharSet {}

#[repr(C)]
pub struct FcObjectType { pub object: *mut c_char, pub type_: FcType }
#[repr(C)]
pub struct FcConstant { pub name: *const FcChar8, pub object: *const c_char, pub value: c_int }

pub type FcResult = c_int;
pub const FcResultMatch: FcResult = 0;
pub const FcResultNoMatch: FcResult = 1;
pub const FcResultTypeMismatch: FcResult = 2;
pub const FcResultNoId: FcResult = 3;
pub const FcResultOutOfMemory: FcResult = 4;

pub type FcValueBinding = c_int;
pub const FcValueBindingWeak: FcValueBinding = 0;
pub const FcValueBindingStrong: FcValueBinding = 1;
pub const FcValueBindingSame: FcValueBinding = 2;

pub enum FcPattern {}
#[repr(C)]
pub struct FcPatternIter { pub dummy1: *mut c_void, pub dummy2: *mut c_void }
pub enum FcLangSet {}
pub enum FcRange {}

#[repr(C)]
pub union FcValueUnion
{
    pub s: *const FcChar8,
    pub i: c_int,
    pub b: FcBool,
    pub d: c_double,
    pub m: *const FcMatrix,
    pub c: *const FcCharSet,
    pub f: *mut c_void,
    pub l: *const FcLangSet,
    pub r: *const FcRange
}
#[repr(C)]
pub struct FcValue { pub type_: FcType, pub u: FcValueUnion }

#[repr(C)]
pub struct FcFontSet { pub nfont: c_int, pub sfont: c_int, pub fonts: *mut *mut FcPattern }
#[repr(C)]
pub struct FcObjectSet { pub nobject: c_int, pub sobject: c_int, pub objects: *mut *const c_char }

pub type FcMatchKind = c_int;
pub const FcMatchPattern: FcMatchKind = 0;
pub const FcMatchFont: FcMatchKind = 1;
pub const FcMatchScan: FcMatchKind = 2;

pub type FcLangResult = c_int;
pub const FcLangEqual: FcLangResult = 0;
pub const FcLangDifferentCountry: FcLangResult = 1;
pub const FcLangDifferentTerritory: FcLangResult = 1;
pub const FcLangDifferentLang: FcLangResult = 2;

pub type FcSetName = c_int;
pub const FcSetSystem: FcSetName = 0;
pub const FcSetApplication: FcSetName = 1;

#[repr(C)]
pub struct FcConfigFileInfoIter
{
    pub dummy1: *mut c_void,
    pub dummy2: *mut c_void,
    pub dummy3: *mut c_void
}

pub enum FcAtomic {}

pub type FcEndian = c_int;
pub const FcEndianBig: FcEndian = 0;
pub const FcEndianLittle: FcEndian = 1;

pub enum FcConfig {}
pub enum FcFileCache {}
pub enum FcBlanks {}
pub enum FcStrList {}
pub enum FcStrSet {}
pub enum FcCache {}

#[link(name = "fontconfig")]
extern "C"
{
    pub fn FcBlanksCreate() -> *mut FcBlanks;
    pub fn FcBlanksDestroy(b: *mut FcBlanks);
    pub fn FcBlanksAdd(b: *mut FcBlanks, ucs4: FcChar32) -> FcBool;
    pub fn FcBlanksIsMember(b: *mut FcBlanks, ucs4: FcChar32) -> FcBool;

    pub fn FcCacheDir(c: *const FcCache) -> *const FcChar8;
    pub fn FcCacheCopySet(c: *const FcCache) -> *mut FcFontSet;
    pub fn FcCacheSubdir(c: *const FcCache, i: c_int) -> *const FcChar8;
    pub fn FcCacheNumSubdir(c: *const FcCache) -> c_int;
    pub fn FcCacheNumFont(c: *const FcCache) -> c_int;
    pub fn FcDirCacheUnlink(dir: *const FcChar8, config: *mut FcConfig) -> FcBool;
    pub fn FcDirCacheValid(cache_file: *const FcChar8) -> FcBool;
    pub fn FcDirCAcheClean(cache_dir: *const FcChar8, verbose: FcBool) -> FcBool;
    pub fn FcCacheCreateTagFile(config: *const FcConfig);
    pub fn FcDirCacheCreateUUID(dir: *mut FcChar8, force: FcBool, config: *mut FcConfig) -> FcBool;
    pub fn FcDirCacheDeleteUUID(dir: *const FcChar8, config: *mut FcConfig) -> FcBool;
    
    pub fn FcConfigHome() -> *mut FcChar8;
    pub fn FcConfigEnableHome(enable: FcBool) -> FcBool;
    pub fn FcConfigFilename(url: *const FcChar8) -> *mut FcChar8;
    pub fn FcConfigCreate() -> *mut FcConfig;
    pub fn FcConfigReference(config: *mut FcConfig) -> *mut FcConfig;
    pub fn FcConfigDestroy(config: *mut FcConfig);
    pub fn FcConfigSetCurrent(config: *mut FcConfig);
    pub fn FcConfigGetCurrent() -> *mut FcConfig;
    pub fn FcConfigUptoDate(config: *mut FcConfig) -> FcBool;
    pub fn FcConfigBuildFonts(config: *mut FcConfig) -> FcBool;
    pub fn FcConfigGetFontDirs(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetConfigDirs(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetConfigFiles(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetCache(config: *mut FcConfig) -> *mut FcChar8;
    pub fn FcConfigGetBlanks(config: *mut FcConfig) -> *mut FcBlanks;
    pub fn FcConfigGetCacheDirs(config: *const FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetRescanInterval(config: *mut FcConfig) -> c_int;
    pub fn FcConfigSetRescanInterval(config: *mut FcConfig, rescan_interval: c_int) -> FcBool;
    pub fn FcConfigGetFonts(config: *mut FcConfig, set: FcSetName) -> *mut FcFontSet;
    pub fn FcConfigAppFontAddFile(config: *mut FcConfig, file: *const FcChar8) -> FcBool;
    pub fn FcConfigAppFontAddDir(config: *mut FcConfig, dir: *const FcChar8) -> FcBool;
    pub fn FcConfigAppFontClear(config: *mut FcConfig);
    pub fn FcConfigSubstituteWithPat(config: *mut FcConfig, p: *mut FcPattern, p_pat: *mut FcPattern, kind: FcMatchKind)
        -> FcBool;
    pub fn FcConfigSubstitute(config: *mut FcConfig, p: *mut FcPattern, kind: FcMatchKind) -> FcBool;
    pub fn FcConfigGetSysRoot(config: *const FcConfig) -> *const FcChar8;
    pub fn FcConfigSetSysRoot(config: *mut FcConfig, sysroot: *const FcChar8);
    pub fn FcConfigFileInfoIterInit(config: *mut FcConfig, iter: *mut FcConfigFileInfoIter);
    pub fn FcConfigFileInfoIterNext(config: *mut FcConfig, iter: *mut FcConfigFileInfoIter) -> FcBool;
    pub fn FcConfigFileInfoIterGet(config: *mut FcConfig, iter: *mut FcConfigFileInfoIter,
        name: *mut *mut FcChar8, description: *mut *mut FcChar8, enabled: *mut FcBool) -> FcBool;
    
    pub fn FcCharSetCreate() -> *mut FcCharSet;
    pub fn FcCharSetDestroy(fcs: *mut FcCharSet);
    pub fn FcCharSetAddChar(fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetDelChar(fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetCopy(src: *mut FcCharSet) -> *mut FcCharSet;
    pub fn FcCharSetEqual(a: *const FcCharSet, b: *const FcCharSet) -> FcBool;
    pub fn FcCharSetItersect(a: *const FcCharSet, b: *const FcCharSet) -> *mut FcCharSet;
    pub fn FcCharSetUnion(a: *const FcCharSet, b: *const FcCharSet) -> *mut FcCharSet;
    pub fn FcCharSetSubtract(a: *const FcCharSet, b: *const FcCharSet) -> *mut FcCharSet;
    pub fn FcCharSetMerge(a: *mut FcCharSet, b: *const FcCharSet, changed: *mut FcBool) -> FcBool;
    pub fn FcCharSetHasChar(fcs: *const FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetCount(a: *const FcCharSet) -> FcChar32;
    pub fn FcCharSetIntersectCount(a: *const FcCharSet, b: *const FcCharSet) -> FcChar32;
    pub fn FcCharSetSubtractCount(a: *const FcCharSet, b: *const FcCharSet) -> FcChar32;
    pub fn FcCharSetIsSubset(a: *const FcCharSet, b: *const FcCharSet) -> FcBool;
    pub fn FcCharSetFirstPage(a: *const FcCharSet, map: *mut FcChar32, next: *mut FcChar32) -> FcChar32;
    pub fn FcCharSetNextPage(a: *const FcCharSet, map: *mut FcChar32, next: *mut FcChar32) -> FcChar32;
    pub fn FcCharSetCoverage(a: *const FcCharSet, page: FcChar32, result: *mut FcChar32) -> FcChar32;

    pub fn FcValuePrint(v: FcValue);
    pub fn FcPatternPrint(p: *const FcPattern);
    pub fn FcFontSetPrint(s: *const FcFontSet);

    pub fn FcGetDefaultLangs() -> *mut FcStrSet;
    pub fn FcDefaultSubstitute(pattern: *mut FcPattern);

    pub fn FcFileIsDir(file: *const FcChar8) -> FcBool;
    pub fn FcFileScan(set: *mut FcFontSet, dirs: *mut FcStrSet, cache: *mut FcFileCache, blanks: *mut FcBlanks,
        file: *const FcChar8, force: FcBool) -> FcBool;
    pub fn FcDirScan(set: *mut FcFontSet, dirs: *mut FcStrSet, cache: *mut FcFileCache, blanks: *mut FcBlanks,
        dir: *const FcChar8, force: FcBool) -> FcBool;
    pub fn FcDirSave(set: *mut FcFontSet, dirs: *mut FcStrSet, dir: *const FcChar8) -> FcBool;
    pub fn FcDirCacheLoad(dir: *const FcChar8, config: *mut FcConfig, cache_file: *mut *mut FcChar8) -> *mut FcCache;
    pub fn FcDirCacheRescan(dir: *const FcChar8, config: *mut FcConfig) -> *mut FcCache;
    pub fn FcDirCacheRead(dir: *const FcChar8, force: FcBool, config: *mut FcConfig) -> *mut FcCache;
    pub fn FcDirCacheLoadFile(cache_file: *const FcChar8, file_stat: *mut stat) -> *mut FcCache;
    pub fn FcDirCacheUnload(cache: *mut FcCache);
    
    pub fn FcFreeTypeQuery(file: *const FcChar8, id: c_uint, blanks: *mut FcBlanks, count: *mut c_int)
        -> *mut FcPattern;
    pub fn FcFreeTypeQueryAll(file: *const FcChar8, id: c_uint, blanks: *mut FcBlanks, count: *mut c_int,
        set: *mut FcFontSet) -> c_uint;
    
    pub fn FcFontSetCreate() -> *mut FcFontSet;
    pub fn FcFontSetDestroy(s: *mut FcFontSet);
    pub fn FcFontSetAdd(s: *mut FcFontSet, font: *mut FcPattern) -> FcBool;

    pub fn FcInitLoadConfig() -> *mut FcConfig;
    pub fn FcInitLoadConfigAndFonts() -> *mut FcConfig;
    pub fn FcInit() -> FcBool;
    pub fn FcFini();
    pub fn FcGetVersion() -> c_int;
    pub fn FcInitReinitialize() -> FcBool;
    pub fn FcInitBringUptoDate() -> FcBool;
    pub fn FcGetLangs() -> *mut FcStrSet;
    pub fn FcLangNormalize(lang: *const FcChar8) -> *mut FcChar8;
    pub fn FcLangGetCharSet(lang: *const FcChar8) -> *const FcCharSet;
    pub fn FcLangSetCreate(lang: *const FcChar8) -> *const FcCharSet;
    pub fn FcLangSetDestroy(ls: *mut FcLangSet);
    pub fn FcLangSetCopy(ls: *const FcLangSet) -> *mut FcLangSet;
    pub fn FcLangSetAdd(ls: *mut FcLangSet, lang: *const FcChar8) -> FcBool;
    pub fn FcLangSetDel(ls: *mut FcLangSet, lang: *const FcChar8) -> FcBool;
    pub fn FcLangSetHasLang(ls: *const FcLangSet, lang: *const FcChar8) -> FcLangResult;
    pub fn FcLangSetCompare(lsa: *const FcLangSet, lsb: *const FcLangSet) -> FcLangResult;
    pub fn FcLangSetContains(lsa: *const FcLangSet, lsb: *const FcLangSet) -> FcBool;
    pub fn FcLangSetEqual(lsa: *const FcLangSet, lsb: *const FcLangSet) -> FcBool;
    pub fn FcLangSetHash(ls: *const FcLangSet) -> FcChar32;
    pub fn FcLangGetLangs(ls: *const FcLangSet) -> *mut FcStrSet;
    pub fn FcLangSetUnion(a: *const FcLangSet, b: *const FcLangSet) -> *mut FcLangSet;
    pub fn FcLangSetSubtract(a: *const FcLangSet, b: *const FcLangSet) -> *mut FcLangSet;

    pub fn FcObjectSetCreate() -> *mut FcObjectSet;
    pub fn FcObjectSetAdd(os: *mut FcObjectSet, object: *const c_char) -> FcBool;
    pub fn FcObjectSetDestroy(os: *mut FcObjectSet);
    //FcObjectSetVaBuild
    pub fn FcObjectSetBuild(first: *const c_char, ...) -> *mut FcObjectSet;
    pub fn FcFontSetList(config: *mut FcConfig, sets: *mut *mut FcFontSet, nsets: c_int,
        p: *mut FcPattern, os: *mut FcObjectSet) -> *mut FcFontSet;
    pub fn FcFontList(config: *mut FcConfig, p: *mut FcPattern, os: *mut FcObjectSet) -> *mut FcFontSet;

    pub fn FcAtomicCreate(file: *const FcChar8) -> *mut FcAtomic;
    pub fn FcAtomicLock(atomic: *mut FcAtomic) -> FcBool;
    pub fn FcAtomicNewFile(atomic: *mut FcAtomic) -> *mut FcChar8;
    pub fn FcAtomicOrigFile(atomic: *mut FcAtomic) -> *mut FcChar8;
    pub fn FcAtomicReplaceOrig(atomic: *mut FcAtomic) -> FcBool;
    pub fn FcAtomicDeleteNew(atomic: *mut FcAtomic);
    pub fn FcAtomicUnlock(atomic: *mut FcAtomic);
    pub fn FcAtomicDestroy(atomic: *mut FcAtomic);

    pub fn FcFontSetMatch(config: *mut FcConfig, sets: *mut *mut FcFontSet, nsets: c_int,
        p: *mut FcPattern, result: *mut FcResult) -> *mut FcPattern;
    pub fn FcFontMatch(config: *mut FcConfig, p: *mut FcPattern, result: *mut FcResult) -> *mut FcPattern;
    pub fn FcFontRenderPrepare(config: *mut FcConfig, pat: *mut FcPattern, font: *mut FcPattern) -> *mut FcPattern;
    pub fn FcFontSetSort(config: *mut FcConfig, sets: *mut *mut FcFontSet, nsets: c_int,
        p: *mut FcPattern, trim: FcBool, csp: *mut *mut FcCharSet, result: *mut FcResult) -> *mut FcFontSet;
    pub fn FcFontSort(config: *mut FcConfig, p: *mut FcPattern, trim: FcBool,
        csp: *mut *mut FcCharSet, result: *mut FcResult) -> *mut FcFontSet;
    pub fn FcFontSetSortDestroy(fs: *mut FcFontSet);

    pub fn FcMatrixCopy(mat: *const FcMatrix) -> *mut FcMatrix;
    pub fn FcMatrixEqual(mat1: *const FcMatrix, mat2: *const FcMatrix) -> FcBool;
    pub fn FcMatrixMultiply(reuslt: *mut FcMatrix, a: *const FcMatrix, b: *const FcMatrix);
    pub fn FcMatrixRotate(m: *mut FcMatrix, c: c_double, s: c_double);
    pub fn FcMatrixScale(m: *mut FcMatrix, sx: c_double, sy: c_double);
    pub fn FcMatrixShear(m: *mut FcMatrix, sh: c_double, sv: c_double);

    pub fn FcNameGetObjectType(object: *const c_char) -> *const FcObjectType;
    pub fn FcNameGetConstant(string: *const FcChar8) -> *const FcConstant;
    pub fn FcNameConstant(string: *const FcChar8, result: *mut c_int) -> FcBool;
    pub fn FcNameParse(name: *const FcChar8) -> *mut FcPattern;
    pub fn FcNameUnparse(pat: *mut FcPattern) -> *mut FcChar8;

    pub fn FcPatternCreate() -> *mut FcPattern;
    pub fn FcPatternDuplicate(p: *const FcPattern) -> *mut FcPattern;
    pub fn FcPatternReference(p: *mut FcPattern);
    pub fn FcPatternFilter(p: *mut FcPattern, os: *const FcObjectSet) -> *mut FcPattern;
    pub fn FcValueDestroy(v: FcValue);
    pub fn FcValueEqual(va: FcValue, vb: FcValue) -> FcBool;
    pub fn FcValueSave(v: FcValue) -> FcValue;
    pub fn FcPatternDestroy(p: *mut FcPattern);
    pub fn FcPatternObjectCount(pat: *const FcPattern) -> c_int;
    pub fn FcPatternEqual(pa: *const FcPattern, pb: *const FcPattern) -> FcBool;
    pub fn FcPatternEqualSubset(pa: *const FcPattern, pb: *const FcPattern, os: *const FcObjectSet) -> FcBool;
    pub fn FcPatternHash(p: *const FcPattern) -> FcChar32;
    pub fn FcPatternAdd(p: *mut FcPattern, object: *const c_char, value: FcValue, append: FcBool) -> FcBool;
    pub fn FcPatternAddWeak(p: *mut FcPattern, object: *const c_char, value: FcValue, append: FcBool) -> FcBool;
    pub fn FcPatternGet(p: *mut FcPattern, object: *const c_char, id: c_int, v: *mut FcValue) -> FcResult;
    pub fn FcPatternGetWithBinding(p: *const FcPattern, object: *const c_char, id: c_int,
        v: *mut FcValue, b: *mut FcValueBinding) -> FcResult;
    pub fn FcPatternDel(p: *mut FcPattern, object: *const c_char) -> FcBool;
    pub fn FcPatternRemove(p: *mut FcPattern, object: *const c_char, id: c_int) -> FcBool;
    pub fn FcPatternAddInteger(p: *mut FcPattern, object: *const c_char, i: c_int) -> FcBool;
    pub fn FcPatternAddDouble(p: *mut FcPattern, object: *const c_char, d: c_double) -> FcBool;
    pub fn FcPatternAddString(p: *mut FcPattern, object: *const c_char, s: *const FcChar8) -> FcBool;
    pub fn FcPatternAddMatrix(p: *mut FcPattern, object: *const c_char, s: *const FcMatrix) -> FcBool;
    pub fn FcPatternAddCharSet(p: *mut FcPattern, object: *const c_char, c: *const FcCharSet) -> FcBool;
    pub fn FcPatternAddBool(p: *mut FcPattern, object: *const c_char, b: FcBool) -> FcBool;
    pub fn FcPatternAddLangSet(p: *mut FcPattern, object: *const c_char, ls: *const FcLangSet) -> FcBool;
    pub fn FcPatternAddRange(p: *mut FcPattern, object: *const c_char, r: *const FcRange) -> FcBool;
    pub fn FcPatternGetInteger(p: *mut FcPattern, object: *const c_char, n: c_int, i: *mut c_int) -> FcBool;
    pub fn FcPatternGetDouble(p: *mut FcPattern, object: *const c_char, n: c_int, d: *mut c_double) -> FcBool;
    pub fn FcPatternGetString(p: *mut FcPattern, object: *const c_char, n: c_int, s: *mut *mut FcChar8) -> FcBool;
    pub fn FcPatternGetMatrix(p: *mut FcPattern, object: *const c_char, n: c_int, s: *mut *mut FcMatrix) -> FcBool;
    pub fn FcPatternGetCharSet(p: *mut FcPattern, object: *const c_char, n: c_int, c: *mut *mut FcCharSet) -> FcBool;
    pub fn FcPatternGetBool(p: *mut FcPattern, object: *const c_char, n: c_int, b: *mut FcBool) -> FcBool;
    pub fn FcPatternGetLangSet(p: *mut FcPattern, object: *const c_char, n: c_int, ls: *mut *mut FcLangSet) -> FcBool;
    pub fn FcPatternGetRange(p: *mut FcPattern, object: *const c_char, n: c_int, r: *mut *mut FcRange) -> FcBool;
    //FcPatternVaBuild
    pub fn FcPatternBuild(p: *mut FcPattern, ...) -> *mut FcPattern;
    pub fn FcPatternFormat(pat: *mut FcPattern, format: *const FcChar8) -> *mut FcChar8;
    
    pub fn FcRangeCreateDouble(begin: c_double, end: c_double) -> *mut FcRange;
    pub fn FcRangeCreateInteger(begin: FcChar32, end: FcChar32) -> *mut FcRange;
    pub fn FcRangeDestroy(range: *mut FcRange);
    pub fn FcRangeCopy(r: *const FcRange) -> *mut FcRange;
    pub fn FcRangeGetDouble(range: *const FcRange, begin: *mut c_double, end: *mut c_double) -> FcBool;
    pub fn FcPatternIterStart(pat: *const FcPattern, iter: *mut FcPatternIter);
    pub fn FcPatternIterNext(pat: *const FcPattern, iter: *mut FcPatternIter) -> FcBool;
    pub fn FcPatternIterEqual(o1: *const FcPattern, i1: *mut FcPatternIter,
        p2: *const FcPattern, i2: *mut FcPatternIter) -> FcBool;
    pub fn FcPatternFindIter(pat: *const FcPattern, iter: *mut FcPatternIter, object: *const c_char) -> FcBool;
    pub fn FcPatternIterIsValid(pat: *const FcPattern, iter: *mut FcPatternIter) -> FcBool;
    pub fn FcPatternIterGetObject(pat: *const FcPattern, iter: *mut FcPatternIter) -> *const c_char;
    pub fn FcPatternIterValudCount(pat: *const FcPattern, iter: *mut FcPatternIter) -> c_int;
    pub fn FcPatternIterGetValue(pat: *const FcPattern, iter: *mut FcPatternIter,
        id: c_int, v: *mut FcValue, b: *mut FcValueBinding) -> FcResult;
    
    pub fn FcWeightFromOpenType(ot_weight: c_int) -> c_int;
    pub fn FcWeightFromOpenTypeDouble(ot_weight: c_double) -> c_double;
    pub fn FcWeightToOpenType(fc_weight: c_int) -> c_int;
    pub fn FcWeightToOpenTypoeDouble(fc_weight: c_double) -> c_double;

    pub fn FcStrCopy(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrCopyFilename(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrPlus(s1: *const FcChar8, s2: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrFree(s: *mut FcChar8);
    pub fn FcStrDowncase(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrCmpIgnoreCase(s1: *const FcChar8, s2: *const FcChar8) -> c_int;
    pub fn FcStrCmp(s1: *const FcChar8, s2: *const FcChar8) -> c_int;
    pub fn FcStrStrIgnoreCase(s1: *const FcChar8, s2: *const FcChar8) -> *const FcChar8;
    pub fn FcStrStr(s1: *const FcChar8, s2: *const FcChar8) -> *const FcChar8;
    pub fn FcUtf8ToUcs4(src_orig: *const FcChar8, dst: *mut FcChar32, len: c_int) -> c_int;
    pub fn FcUtf8Len(string: *const FcChar8, len: c_int, n_char: *mut c_int, wchar: *mut c_int) -> FcBool;
    pub fn FcUcs4ToUtf8(ucs4: FcChar32, dest: *mut FcChar8) -> c_int;
    pub fn FcUtf16ToUcs4(src_orig: *const FcChar8, endian: FcEndian, dst: *mut FcChar32, len: c_int) -> c_int;
    pub fn FcUtf16Len(string: *const FcChar8, endian: FcEndian, len: c_int, nchar: *mut c_int, wchar: *mut c_int)
        -> FcBool;
    pub fn FcStrBuildFilename(path: *const FcChar8, ...) -> *mut FcChar8;
    pub fn FcStrDirname(file: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrBasename(file: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrSetCreate() -> *mut FcStrSet;
    pub fn FcStrSetMember(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetEqual(sa: *mut FcStrSet, sb: *mut FcStrSet) -> FcBool;
    pub fn FcStrSetAdd(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetAddFilename(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetDel(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetDestroy(set: *mut FcStrSet);
    pub fn FcStrListCreate(set: *mut FcStrSet) -> *mut FcStrList;
    pub fn FcStrListFirst(list: *mut FcStrList);
    pub fn FcStrListNext(list: *mut FcStrList) -> *mut FcChar8;
    pub fn FcStrListDone(list: *mut FcStrList);

    pub fn FcConfigParseAndLoad(config: *mut FcConfig, file: *const FcChar8, complain: FcBool) -> FcBool;
    pub fn FcCofnigParseAndLoadFromMemory(config: *mut FcConfig, buffer: *const FcChar8, complain: FcBool) -> FcBool;
}

pub const FC_CHARSET_MAP_SIZE: usize = 256 / 32;
pub const FC_CHARSET_DONE: FcChar32 = 0xffff_ffff; // -1
pub const FC_UTF8_MAX_LEN: usize = 6;
