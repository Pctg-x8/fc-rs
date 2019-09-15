
use freetype2::*;
use super::*;
use libc::*;

extern "C"
{
	pub fn FcFreeTypeCharIndex(face: FT_Face, ucs4: FcChar32) -> FT_UInt;
	pub fn FcFreeTypeCharSetAndSpacing(face: FT_Face, blanks: *mut FcBlanks, spacing: *mut c_int) -> *mut FcCharSet;
	pub fn FcFreeTypeCharSet(face: FT_Face, blanks: *mut FcBlanks) -> *mut FcCharSet;
	pub fn FcPatternGetFTFace(p: *const FcPattern, object: *const c_char, n: c_int, f: *mut FT_Face) -> FcResult;
	pub fn FcPatternAddFTFace(p: *mut FcPattern, object: *const c_char, f: FT_Face) -> FcBool;
	pub fn FcFreeTypeQueryface(face: FT_Face, file: *const FcChar8, id: c_uint, blanks: *mut FcBlanks)
		-> *mut FcPattern;
}
