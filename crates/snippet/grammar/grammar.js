module.exports = grammar({
    name: 'snippet',

    rules: {
        snippet: $ => repeat1($._any),

        _any: $ => choice(
            $.tabstop,
            $.placeholder,
            $.text
        ),

        tabstop: $ => choice(
            seq('$', $.int),
            seq('${', $.int, '}'),
        ),

        placeholder: $ => seq('${', $.int, ':', $.snippet, '}'),

        int: $ => /[0-9]+/,

        text: $ => choice($._raw_curly, $._plain_text),
        _raw_curly: $ => token(prec(-1, /}+/)),
        _plain_text: $ => /([^$}]|\\[$\\}])+/,
    }
})