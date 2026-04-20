+++
repository_url = "https://github.com/pallets/flask"
revision = "2fec0b206c6e83ea813ab26597e15c96fab08be7"
+++

## Edit History

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -356,3 +356,6 @@
     cookie = rv.headers["set-cookie"].lower()
     assert "samesite=lax" in cookie


+de
+
+
 def test_missing_session(app):
```

// User accepted prediction:
```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -358,6 +358,14 @@


-de
+def test_session_cookie_httponly(app, client):
+    app.config["SESSION_COOKIE_HTTPONLY"] = True
+
+    @app.route("/")
+    def index():
+        flask.session["testing"] = 42
+        return "Hello World"
+
+    rv = client.get("/")
+    assert "httponly" in rv.headers["set-cookie"].lower()


 def test_missing_session(app):
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -358,14 +358,14 @@


-def test_session_cookie_httponly(app, client):
+def test_session_cookie_secur(app, client):
     app.config["SESSION_COOKIE_HTTPONLY"] = True
```

## Cursor Position

```tests/test_basic.py
    cookie = rv.headers["set-cookie"].lower()
    assert "samesite=lax" in cookie


def test_session_cookie_secur(app, client):
#                            ^[CURSOR_POSITION]
    app.config["SESSION_COOKIE_HTTPONLY"] = True

    @app.route("/")
    def index():
        flask.session["testing"] = 42
        return "Hello World"

    rv = client.get("/")
    assert "httponly" in rv.headers["set-cookie"].lower()


def test_missing_session(app):
```

## Expected Patch

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -358,14 +358,14 @@
-def test_session_cookie_secur(app, client):
-    app.config["SESSION_COOKIE_HTTPONLY"] = True
+def test_session_cookie_secure(app, client):
+    app.config["SESSION_COOKIE_SECURE"] = True

     @app.route("/")
     def index():
         flask.session["testing"] = 42
         return "Hello World"

     rv = client.get("/")
-    assert "httponly" in rv.headers["set-cookie"].lower()
+    assert "secure" in rv.headers["set-cookie"].lower()
```
