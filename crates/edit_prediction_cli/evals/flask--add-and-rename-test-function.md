+++
repository_url = "https://github.com/pallets/flask"
revision = "2fec0b206c6e83ea813ab26597e15c96fab08be7"
+++

## Edit History

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1376,5 +1376,8 @@
 def test_static_files(app, client):
     rv = client.get("/static/index.html")
     assert rv.status_code == 200
     assert rv.data.strip() == b"<h1>Hello World!</h1>"
     with app.test_request_context():
         assert flask.url_for("static", filename="index.html") == "/static/index.html"
     rv.close()


+de
+
+
 def test_static_url_path():
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_():
+    pass


 def test_static_url_path():
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-def test_():
+def test_static_file_not_found():
     pass


 def test_static_url_path():
```

## Cursor Position

```tests/test_basic.py
def test_static_file_not_found():
#                             ^[CURSOR_POSITION]
    pass


def test_static_url_path():
```

## Expected Patch

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-def test_static_file_not_found():
-    pass
+def test_static_file_not_found(app, client):
+    rv = client.get("/static/non_existent.html")
+    assert rv.status_code == 404
+    rv.close()
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-def test_static_file_not_found():
-    pass
+def test_static_file_not_found(app, client):
+    rv = client.get("/static/not_found.html")
+    assert rv.status_code == 404
+    rv.close()
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1376,8 +1376,13 @@
-def test_static_file_not_found():
-    pass
+def test_static_file_not_found(app, client):
+    rv = client.get("/static/nonexistent.html")
+    assert rv.status_code == 404
+    assert rv.data.strip() == b"<h1>Not Found</h1>"
+    with app.test_request_context():
+        pytest.raises(BuildError, flask.url_for, "static", filename="nonexistent.html")
+    rv.close()


 def test_static_url_path():
```
