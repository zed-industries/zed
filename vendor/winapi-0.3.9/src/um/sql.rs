// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use um::sqltypes::{
    SQLHANDLE, SQLHDBC, SQLHENV, SQLHSTMT, SQLINTEGER, SQLLEN, SQLPOINTER, SQLRETURN, SQLSMALLINT,
    SQLUSMALLINT,
};
pub const SQL_NULL_DATA: SQLLEN = -1;
pub const SQL_DATA_AT_EXEC: SQLLEN = -2;
pub const SQL_SUCCESS: SQLRETURN = 0;
pub const SQL_SUCCESS_WITH_INFO: SQLRETURN = 1;
pub const SQL_NO_DATA: SQLRETURN = 100;
pub const SQL_PARAM_DATA_AVAILABLE: SQLRETURN = 101;
pub const SQL_ERROR: SQLRETURN = -1;
pub const SQL_INVALID_HANDLE: SQLRETURN = -2;
pub const SQL_STILL_EXECUTING: SQLRETURN = 2;
pub const SQL_NEED_DATA: SQLRETURN = 99;
pub const SQL_NTS: SQLSMALLINT = -3;
pub const SQL_MAX_MESSAGE_LENGTH: usize = 512;
pub const SQL_DATE_LEN: usize = 10;
pub const SQL_TIME_LEN: usize = 8;
pub const SQL_TIMESTAMP_LEN: usize = 19;
pub const SQL_HANDLE_ENV: SQLSMALLINT = 1;
pub const SQL_HANDLE_DBC: SQLSMALLINT = 2;
pub const SQL_HANDLE_STMT: SQLSMALLINT = 3;
pub const SQL_HANDLE_DESC: SQLSMALLINT = 4;
pub const SQL_ATTR_OUTPUT_NTS: SQLINTEGER = 10001;
pub const SQL_ATTR_AUTO_IPD: SQLINTEGER = 10001;
pub const SQL_ATTR_METADATA_ID: SQLINTEGER = 10014;
pub const SQL_ATTR_APP_ROW_DESC: SQLINTEGER = 10010;
pub const SQL_ATTR_APP_PARAM_DESC: SQLINTEGER = 10011;
pub const SQL_ATTR_IMP_ROW_DESC: SQLINTEGER = 10012;
pub const SQL_ATTR_IMP_PARAM_DESC: SQLINTEGER = 10013;
pub const SQL_ATTR_CURSOR_SCROLLABLE: SQLINTEGER = -1;
pub const SQL_ATTR_CURSOR_SENSITIVITY: SQLINTEGER = -2;
pub const SQL_UNKNOWN_TYPE: SQLSMALLINT = 0;
pub const SQL_CHAR: SQLSMALLINT = 1;
pub const SQL_NUMERIC: SQLSMALLINT = 2;
pub const SQL_DECIMAL: SQLSMALLINT = 3;
pub const SQL_INTEGER: SQLSMALLINT = 4;
pub const SQL_SMALLINT: SQLSMALLINT = 5;
pub const SQL_FLOAT: SQLSMALLINT = 6;
pub const SQL_REAL: SQLSMALLINT = 7;
pub const SQL_DOUBLE: SQLSMALLINT = 8;
pub const SQL_DATETIME: SQLSMALLINT = 9;
pub const SQL_VARCHAR: SQLSMALLINT = 12;
pub const SQL_TYPE_DATE: SQLSMALLINT = 91;
pub const SQL_TYPE_TIME: SQLSMALLINT = 92;
pub const SQL_TYPE_TIMESTAMP: SQLSMALLINT = 93;
pub const SQL_NO_NULLS: SQLSMALLINT = 0;
pub const SQL_NULLABLE: SQLSMALLINT = 1;
pub const SQL_NULLABLE_UNKNOWN: SQLSMALLINT = 2;
pub const SQL_CLOSE: SQLUSMALLINT = 0;
pub const SQL_DROP: SQLUSMALLINT = 1;
pub const SQL_UNBIND: SQLUSMALLINT = 2;
pub const SQL_RESET_PARAMS: SQLUSMALLINT = 3;
pub const SQL_NULL_HANDLE: SQLHANDLE = 0 as SQLHANDLE;
extern "system" {
    pub fn SQLAllocHandle(
        handleType: SQLSMALLINT,
        inputHandle: SQLHANDLE,
        outputHandle: *mut SQLHANDLE,
    ) -> SQLRETURN;
    pub fn SQLDisconnect(
        connectionHandle: SQLHDBC,
    ) -> SQLRETURN;
    pub fn SQLFetch(
        statementHandle: SQLHSTMT,
    ) -> SQLRETURN;
    pub fn SQLFreeHandle(
        handleType: SQLSMALLINT,
        handle: SQLHANDLE,
    ) -> SQLRETURN;
    pub fn SQLFreeStmt(
        statementHandle: SQLHSTMT,
        option: SQLUSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLGetData(
        statementHandle: SQLHSTMT,
        columnNumber: SQLUSMALLINT,
        targetType: SQLSMALLINT,
        targetValue: SQLPOINTER,
        bufferLength: SQLLEN,
        strLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;
    pub fn SQLNumResultCols(
        statementHandle: SQLHSTMT,
        columnCount: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLRowCount(
        statementHandle: SQLHSTMT,
        rowCount: *mut SQLLEN,
    ) -> SQLRETURN;
    pub fn SQLSetConnectAttr(
        connectionHandle: SQLHDBC,
        attribute: SQLINTEGER,
        value: SQLPOINTER,
        stringLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLSetEnvAttr(
        environmentHandle: SQLHENV,
        attribute: SQLINTEGER,
        value: SQLPOINTER,
        stringLength: SQLINTEGER,
    ) -> SQLRETURN;
}
