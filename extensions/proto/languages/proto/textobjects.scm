(message (message_body
    "{"
    (_)* @class.inside
    "}")) @class.around
(enum (enum_body
    "{"
    (_)* @class.inside
    "}")) @class.around
(service
    "service"
    (_)
    "{"
    (_)* @class.inside
    "}") @class.around

(rpc) @function.around

(comment)+ @comment.around
