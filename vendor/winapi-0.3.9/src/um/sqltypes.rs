// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the types used in ODBC
use ctypes::*;
#[cfg(target_pointer_width = "64")]
use shared::basetsd::{INT64, UINT64};
use shared::guiddef::GUID;
use shared::windef::HWND;
pub type SQLCHAR = c_uchar;
pub type SQLSCHAR = c_schar;
pub type SQLDATE = c_uchar;
pub type SQLDECIMAL = c_uchar;
pub type SQLDOUBLE = c_double;
pub type SQLFLOAT = c_double;
pub type SQLINTEGER = c_long;
pub type SQLUINTEGER = c_ulong;
#[cfg(target_pointer_width = "64")]
pub type SQLLEN = INT64;
#[cfg(target_pointer_width = "64")]
pub type SQLULEN = UINT64;
#[cfg(target_pointer_width = "64")]
pub type SQLSETPOSIROW = UINT64;
#[cfg(target_pointer_width = "32")]
pub type SQLLEN = SQLINTEGER;
#[cfg(target_pointer_width = "32")]
pub type SQLULEN = SQLUINTEGER;
#[cfg(target_pointer_width = "32")]
pub type SQLSETPOSIROW = SQLUSMALLINT;
pub type SQLROWCOUNT = SQLULEN;
pub type SQLROWSETSIZE = SQLULEN;
pub type SQLTRANSID = SQLULEN;
pub type SQLROWOFFSET = SQLLEN;
pub type SQLNUMERIC = c_uchar;
pub type SQLPOINTER = *mut c_void;
pub type SQLREAL = c_float;
pub type SQLSMALLINT = c_short;
pub type SQLUSMALLINT = c_ushort;
pub type SQLTIME = c_uchar;
pub type SQLTIMESTAMP = c_uchar;
pub type SQLVARCHAR = c_uchar;
pub type SQLRETURN = SQLSMALLINT;
pub type SQLHANDLE = *mut c_void;
pub type SQLHENV = SQLHANDLE;
pub type SQLHDBC = SQLHANDLE;
pub type SQLHSTMT = SQLHANDLE;
pub type SQLHDESC = SQLHANDLE;
//pub type UCHAR = c_uchar;
pub type SCHAR = c_schar;
//pub type SQLSCHAR = SCHAR;
pub type SDWORD = c_long;
pub type SWORD = c_short;
pub type UDWORD = c_ulong;
//pub type UWORD = c_ushort;
//#[cfg(target_pointer_width = "32")]
//pub type SQLUINTEGER = UDWORD;
pub type SLONG = c_long;
pub type SSHORT = c_short;
//pub type ULONG = c_ulong;
//pub type USHORT = c_ushort;
pub type SDOUBLE = c_double;
pub type LDOUBLE = c_double;
pub type SFLOAT = c_float;
pub type PTR = *mut c_void;
pub type HENV = *mut c_void;
pub type HDBC = *mut c_void;
pub type HSTMT = *mut c_void;
pub type RETCODE = c_short;
pub type SQLHWND = HWND;
STRUCT!{struct DATE_STRUCT {
    year: SQLSMALLINT,
    month: SQLUSMALLINT,
    day: SQLUSMALLINT,
}}
pub type SQL_DATE_STRUCT = DATE_STRUCT;
STRUCT!{struct TIME_STRUCT {
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
}}
pub type SQL_TIME_STRUCT = TIME_STRUCT;
STRUCT!{struct TIMESTAMP_STRUCT {
    year: SQLSMALLINT,
    month: SQLUSMALLINT,
    day: SQLUSMALLINT,
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
    fraction: SQLUINTEGER,
}}
pub type SQL_TIMESTAMP_STRUCT = TIMESTAMP_STRUCT;
ENUM!{enum SQLINTERVAL {
    SQL_IS_YEAR = 1,
    SQL_IS_MONTH = 2,
    SQL_IS_DAY = 3,
    SQL_IS_HOUR = 4,
    SQL_IS_MINUTE = 5,
    SQL_IS_SECOND = 6,
    SQL_IS_YEAR_TO_MONTH = 7,
    SQL_IS_DAY_TO_HOUR = 8,
    SQL_IS_DAY_TO_MINUTE = 9,
    SQL_IS_DAY_TO_SECOND = 10,
    SQL_IS_HOUR_TO_MINUTE = 11,
    SQL_IS_HOUR_TO_SECOND = 12,
    SQL_IS_MINUTE_TO_SECOND = 13,
}}
STRUCT!{struct SQL_YEAR_MONTH_STRUCT {
    year: SQLUINTEGER,
    month: SQLUINTEGER,
}}
STRUCT!{struct SQL_DAY_SECOND_STRUCT {
    day: SQLUINTEGER,
    hour: SQLUINTEGER,
    minute: SQLUINTEGER,
    second: SQLUINTEGER,
    fraction: SQLUINTEGER,
}}
UNION!{union SQL_INTERVAL_STRUCT_intval {
    [u32; 5],
    year_month year_month_mut: SQL_YEAR_MONTH_STRUCT,
    day_second day_second_mut: SQL_DAY_SECOND_STRUCT,
}}
STRUCT!{struct SQL_INTERVAL_STRUCT {
    interval_type: SQLINTERVAL,
    interval_sign: SQLSMALLINT,
    intval: SQL_INTERVAL_STRUCT_intval,
}}
pub type ODBCINT64 = __int64;
pub type SQLBIGINT = ODBCINT64;
pub type SQLUBIGINT = __uint64;
pub const SQL_MAX_NUMERIC_LEN: usize = 16;
STRUCT!{struct SQL_NUMERIC_STRUCT {
    precision: SQLCHAR,
    scale: SQLSCHAR,
    sign: SQLCHAR,
    val: [SQLCHAR; SQL_MAX_NUMERIC_LEN],
}}
pub type SQLGUID = GUID;
pub type BOOKMARK = SQLULEN;
pub type SQLWCHAR = wchar_t;
