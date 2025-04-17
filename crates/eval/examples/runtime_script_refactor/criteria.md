1. **Added RuntimeCommands import and WebElement to page.py**
The changes add an import for `RuntimeCommands` and `WebElement` to `page.py`. The `execute_js_script` method is renamed to `execute_script` and enhanced to support execution in the context of a WebElement. The method now uses `RuntimeCommands` for script evaluation.
2. **Refactored Runtime-related commands from DomCommands to new RuntimeCommands class**
The changes move all Runtime-related command templates and methods from `DomCommands` in `dom.py` to a new `runtime.py` file. This includes `EVALUATE_TEMPLATE`, `CALL_FUNCTION_ON_TEMPLATE`, `GET_PROPERTIES`, and their associated methods. The DomCommands class now uses RuntimeCommands for JavaScript evaluation.
3. **Added Scripts constants and enhanced WebElement functionality**
The changes add a new `Scripts` class to `constants.py` containing JavaScript snippets for common operations. The `element.py` file is significantly enhanced with new methods for script execution, visibility checking, and improved click handling. New exceptions are added to `exceptions.py` for better error handling.
