+++
repository_url = "https://github.com/pallets/flask"
revision = "2fec0b206c6e83ea813ab26597e15c96fab08be7"
+++

# 2026-02-16 09:02:34

## Edit History

```diff
--- a/tests/test_reqctx.py
+++ b/tests/test_reqctx.py
@@ -132,6 +132,7 @@
         ctx.pop()
+
+


 def test_manual_context_binding(app):
     @app.route("/")
     def index():
@@ -143,7 +144,6 @@
     ctx.pop()
     with pytest.raises(RuntimeError):
         index()
--- a/tests/test_reqctx.py
+++ b/tests/test_reqctx.py
@@ -131,7 +131,7 @@
     finally:
         ctx.pop()

-
+de


 def test_manual_context_binding(app):
     @app.route("/")
```

## Cursor Position

```tests/test_reqctx.py
    assert not flask.has_request_context()
    ctx = app.test_request_context()
    ctx.push()
    try:
        assert flask.request
        assert flask.has_request_context()
    finally:
        ctx.pop()

de
# ^[CURSOR_POSITION]


def test_manual_context_binding(app):
    @app.route("/")
    def index():
        return f"Hello {flask.request.args['name']}!"

    ctx = app.test_request_context("/?name=World")
    ctx.push()
```

## Expected Patch

```diff
--- a/tests/test_reqctx.py
+++ b/tests/test_reqctx.py
@@ -125,18 +125,18 @@
     assert not flask.has_request_context()
     ctx = app.test_request_context()
     ctx.push()
     try:
         assert flask.request
         assert flask.has_request_context()
     finally:
         ctx.pop()

-de
+def test_something(app):
#    --------------^[SELECTION]
+
#    ^[CURSOR_POSITION]


 def test_manual_context_binding(app):
     @app.route("/")
     def index():
         return f"Hello {flask.request.args['name']}!"

     ctx = app.test_request_context("/?name=World")
     ctx.push()
```
