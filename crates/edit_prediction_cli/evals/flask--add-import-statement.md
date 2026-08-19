+++
repository_url = "https://github.com/pallets/flask"
revision = "2fec0b206c6e83ea813ab26597e15c96fab08be7"
+++

## Edit History

```diff
--- a/src/flask/logging.py
+++ b/src/flask/logging.py
@@ -4,7 +4,7 @@
 import sys
 import typing as t

-from werkzeug.local import LocalProxy
+imfrom werkzeug.local import LocalProxy

 from .globals import request

```

## Cursor Position

```src/flask/logging.py
from __future__ import annotations

import logging
import sys
import typing as t

imfrom werkzeug.local import LocalProxy
# ^[CURSOR_POSITION]

from .globals import request

if t.TYPE_CHECKING:  # pragma: no cover
    from .sansio.app import App
```

## Expected Patch

```diff
--- a/src/flask/logging.py
+++ b/src/flask/logging.py
@@ -1,21 +1,21 @@
 from __future__ import annotations

 import logging
 import sys
 import typing as t

-imfrom werkzeug.local import LocalProxy
+import
#       ^[CURSOR_POSITION]
+from werkzeug.local import LocalProxy

 from .globals import request
```

```diff
--- a/src/flask/logging.py
+++ b/src/flask/logging.py
@@ -1,21 +1,21 @@
 from __future__ import annotations

 import logging
 import sys
 import typing as t
-
-imfrom werkzeug.local import LocalProxy
+import werkzeug
#               ^[CURSOR_POSITION]
+from werkzeug.local import LocalProxy

 from .globals import request
```
