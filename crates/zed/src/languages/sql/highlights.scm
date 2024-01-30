(string) @string
(number) @number
(comment) @comment

(function_call
    function: (identifier) @function)

[
  (NULL)
  (TRUE)
  (FALSE)
] @constant.builtin

([
  (type_cast
   (type (identifier) @type.builtin))
  (create_type_statement
    (type_spec_composite (type (identifier) @type.builtin)))
  (create_table_statement
   (table_parameters
     (table_column (type (identifier) @type.builtin))))
 ]
 (#match?
   @type.builtin
    "^(bigint|BIGINT|int8|INT8|bigserial|BIGSERIAL|serial8|SERIAL8|bit|BIT|varbit|VARBIT|boolean|BOOLEAN|bool|BOOL|box|BOX|bytea|BYTEA|character|CHARACTER|char|CHAR|varchar|VARCHAR|cidr|CIDR|circle|CIRCLE|date|DATE|float8|FLOAT8|inet|INET|integer|INTEGER|int|INT|int4|INT4|interval|INTERVAL|json|JSON|jsonb|JSONB|line|LINE|lseg|LSEG|macaddr|MACADDR|money|MONEY|numeric|NUMERIC|decimal|DECIMAL|path|PATH|pg_lsn|PG_LSN|point|POINT|polygon|POLYGON|real|REAL|float4|FLOAT4|smallint|SMALLINT|int2|INT2|smallserial|SMALLSERIAL|serial2|SERIAL2|serial|SERIAL|serial4|SERIAL4|text|TEXT|time|TIME|time|TIME|timestamp|TIMESTAMP|tsquery|TSQUERY|tsvector|TSVECTOR|txid_snapshot|TXID_SNAPSHOT|enum|ENUM|range|RANGE)$"))

(identifier) @variable

[
  "::"
  "<"
  "<="
  "<>"
  "="
  ">"
  ">="
] @operator

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  ";"
  "."
] @punctuation.delimiter

[
  (type)
  (array_type)
] @type

[
 (primary_key_constraint)
 (unique_constraint)
 (null_constraint)
] @keyword

[
  "ADD"
  "ALTER"
  "AND"
  "AS"
  "AUTO_INCREMENT"
  "COLUMN"
  "CREATE_DOMAIN"
  "CREATE_EXTENSION"
  "CREATE_SCHEMA"
  "CREATE_TYPE"
  "CREATE"
  "DATABASE"
  "DEFAULT"
  "DROP"
  "FROM"
  "GRANT"
  "GROUP_BY"
  "IF_EXISTS"
  "IF_NOT_EXISTS"
  "IN"
  "INDEX"
  "INNER"
  "INSERT"
  "INTO"
  "JOIN"
  "LANGUAGE"
  "LEFT"
  "LOCAL"
  "NOT"
  "NULL"
  "ON"
  "OR"
  "ORDER_BY"
  "OUTER"
  "PRIMARY_KEY"
  "PUBLIC"
  "REFERENCES"
  "SCHEMA"
  "SELECT"
  "SESSION"
  "SET"
  "TABLE"
  "TABLE"
  "TEMPORARY"
  "TIME_ZONE"
  "TO"
  "UNIQUE"
  "UPDATE"
  "USAGE"
  "VALUES"
  "WHERE"
  "WITH"
  "WITHOUT"
] @keyword