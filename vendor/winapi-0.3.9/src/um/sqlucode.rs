// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the ODBC Core unicode functions
use um::sqltypes::{
    SQLCHAR, SQLHANDLE, SQLHDBC, SQLHSTMT, SQLHWND, SQLINTEGER, SQLRETURN, SQLSMALLINT, SQLULEN,
    SQLUSMALLINT, SQLWCHAR
};
pub const SQL_WCHAR: SQLSMALLINT = -8;
pub const SQL_WVARCHAR: SQLSMALLINT = -9;
pub const SQL_WLONGVARCHAR: SQLSMALLINT = -10;
pub const SQL_C_WCHAR: SQLSMALLINT = SQL_WCHAR;
extern "system" {
    pub fn SQLConnectW(
        connectionHandle: SQLHDBC,
        serverName: *const SQLWCHAR,
        nameLength1: SQLSMALLINT,
        userName: *const SQLWCHAR,
        nameLength2: SQLSMALLINT,
        authentication: *const SQLWCHAR,
        nameLength3: SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDescribeColW(
        statementHandle: SQLHSTMT,
        columnNumber: SQLUSMALLINT,
        columnName: *mut SQLWCHAR,
        bufferLength: SQLSMALLINT,
        nameLength: *mut SQLSMALLINT,
        dataType: *mut SQLSMALLINT,
        columnSize: *mut SQLULEN,
        decimalDigits: *mut SQLSMALLINT,
        nullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLExecDirectW(
        statementHandle: SQLHSTMT,
        statementText: *const SQLWCHAR,
        textLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLGetDiagRecW(
        handleType: SQLSMALLINT,
        handle: SQLHANDLE,
        recNumber: SQLSMALLINT,
        sqlstate: *mut SQLWCHAR,
        nativeError: *mut SQLINTEGER,
        messageText: *mut SQLWCHAR,
        bufferLength: SQLSMALLINT,
        textLength: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDriverConnectW(
        hdbc: SQLHDBC,
        hwnd: SQLHWND,
        szConnStrIn: *const SQLWCHAR,
        cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLWCHAR,
        cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT,
        fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLConnectA(
        connectionHandle: SQLHDBC,
        serverName: *const SQLCHAR,
        nameLength1: SQLSMALLINT,
        userName: *const SQLCHAR,
        nameLength2: SQLSMALLINT,
        authentication: *const SQLCHAR,
        nameLength3: SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDescribeColA(
        statementHandle: SQLHSTMT,
        columnNumber: SQLUSMALLINT,
        columnName: *mut SQLCHAR,
        bufferLength: SQLSMALLINT,
        nameLength: *mut SQLSMALLINT,
        dataType: *mut SQLSMALLINT,
        columnSize: *mut SQLULEN,
        decimalDigits: *mut SQLSMALLINT,
        nullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLExecDirectA(
        statementHandle: SQLHSTMT,
        statementText: *const SQLCHAR,
        textLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLGetDiagRecA(
        handleType: SQLSMALLINT,
        handle: SQLHANDLE,
        recNumber: SQLSMALLINT,
        sqlstate: *mut SQLCHAR,
        nativeError: *mut SQLINTEGER,
        messageText: *mut SQLCHAR,
        bufferLength: SQLSMALLINT,
        textLength: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDriverConnectA(
        hdbc: SQLHDBC,
        hwnd: SQLHWND,
        szConnStrIn: *const SQLCHAR,
        cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLCHAR,
        cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT,
        fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
}
