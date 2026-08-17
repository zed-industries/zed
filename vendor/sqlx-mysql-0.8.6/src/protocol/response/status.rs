// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/mysql__com_8h.html#a1d854e841086925be1883e4d7b4e8cad
// https://mariadb.com/kb/en/library/mariadb-connectorc-types-and-definitions/#server-status
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct Status: u16 {
        // Is raised when a multi-statement transaction has been started, either explicitly,
        // by means of BEGIN or COMMIT AND CHAIN, or implicitly, by the first
        // transactional statement, when autocommit=off.
        const SERVER_STATUS_IN_TRANS = 1;

        // Autocommit mode is set
        const SERVER_STATUS_AUTOCOMMIT = 2;

        // Multi query - next query exists.
        const SERVER_MORE_RESULTS_EXISTS = 8;

        const SERVER_QUERY_NO_GOOD_INDEX_USED = 16;
        const SERVER_QUERY_NO_INDEX_USED = 32;

        // When using COM_STMT_FETCH, indicate that current cursor still has result
        const SERVER_STATUS_CURSOR_EXISTS = 64;

        // When using COM_STMT_FETCH, indicate that current cursor has finished to send results
        const SERVER_STATUS_LAST_ROW_SENT = 128;

        // Database has been dropped
        const SERVER_STATUS_DB_DROPPED = (1 << 8);

        // Current escape mode is "no backslash escape"
        const SERVER_STATUS_NO_BACKSLASH_ESCAPES = (1 << 9);

        // A DDL change did have an impact on an existing PREPARE (an automatic
        // re-prepare has been executed)
        const SERVER_STATUS_METADATA_CHANGED = (1 << 10);

        // Last statement took more than the time value specified
        // in server variable long_query_time.
        const SERVER_QUERY_WAS_SLOW = (1 << 11);

        // This result-set contain stored procedure output parameter.
        const SERVER_PS_OUT_PARAMS = (1 << 12);

        // Current transaction is a read-only transaction.
        const SERVER_STATUS_IN_TRANS_READONLY = (1 << 13);

        // This status flag, when on, implies that one of the state information has changed
        // on the server because of the execution of the last statement.
        const SERVER_SESSION_STATE_CHANGED = (1 << 14);
    }
}
