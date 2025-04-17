1. The changes add an import for `URIUtil` and modify the URL creation in `OSGiApp.java` to use `URIUtil.correctURI()` for proper URI handling. The modification ensures correct URI formatting before converting to URL.
2. The changes add an import for `URIUtil` and modify the URI creation in `Util.java` to use `URIUtil.correctURI()` when handling file paths. This ensures proper URI formatting for paths starting with "file:/".
3. The changes in both `WebInfConfiguration.java` files (EE10 and EE9 versions) refactor the war file handling logic. The modifications:
   - Add explanatory comments about looking for sibling directories
   - Change how the war path is obtained (using webApp.getPath() instead of creating new resources)
   - Restructure the conditional logic for better clarity
   - Maintain the same functionality but with improved safety checks and documentation
