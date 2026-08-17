"use strict";
window.editors = [];
(function(editors) {
    if (typeof(ace) === 'undefined' || !ace) {
        return;
    }

    Array.from(document.querySelectorAll('.editable')).forEach(function(editable) {
        let display_line_numbers = window.playground_line_numbers || false;

        let editor = ace.edit(editable);
            editor.setOptions({
            highlightActiveLine: false,
            showPrintMargin: false,
            showLineNumbers: display_line_numbers,
            showGutter: display_line_numbers,
            maxLines: Infinity,
            fontSize: "0.875em" // please adjust the font size of the code in general.css
        });

        editor.$blockScrolling = Infinity;

        editor.getSession().setMode("ace/mode/rust");

        editor.originalCode = editor.getValue();

        editors.push(editor);
    });
})(window.editors);
