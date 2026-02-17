+++
repository_url = "https://github.com/pallets/flask"
revision = "2fec0b206c6e83ea813ab26597e15c96fab08be7"
+++

# flask--add-for-loop

## Edit History

```diff
--- a/src/flask/app.py
+++ b/src/flask/app.py
@@ -1361,7 +1361,7 @@
         for func in reversed(self.teardown_appcontext_funcs):
             self.ensure_sync(func)(exc)
+        for

         appcontext_tearing_down.send(self, _async_wrapper=self.ensure_sync, exc=exc)
```

## Cursor Position

```src/flask/app.py
        .. versionadded:: 0.9
        """
        if exc is _sentinel:
            exc = sys.exc_info()[1]

        for func in reversed(self.teardown_appcontext_funcs):
            self.ensure_sync(func)(exc)
        for
        #  ^[CURSOR_POSITION]

        appcontext_tearing_down.send(self, _async_wrapper=self.ensure_sync, exc=exc)
```

## Expected Patch

```diff
--- a/src/flask/app.py
+++ b/src/flask/app.py
@@ -1357,15 +1357,15 @@
         .. versionadded:: 0.9
         """
         if exc is _sentinel:
             exc = sys.exc_info()[1]

         for func in reversed(self.teardown_appcontext_funcs):
             self.ensure_sync(func)(exc)
         for

         appcontext_tearing_down.send(self, _async_wrapper=self.ensure_sync, exc=exc)
```

```diff
--- a/src/flask/app.py
+++ b/src/flask/app.py
@@ -1357,15 +1357,15 @@
         .. versionadded:: 0.9
         """
         if exc is _sentinel:
             exc = sys.exc_info()[1]

         for func in reversed(self.teardown_appcontext_funcs):
             self.ensure_sync(func)(exc)
-        for
+        for func in iterable:
#            ----^[SELECTION]
#                    --------^[SELECTION]
+
#            ^[CURSOR]

         appcontext_tearing_down.send(self, _async_wrapper=self.ensure_sync, exc=exc)
```
