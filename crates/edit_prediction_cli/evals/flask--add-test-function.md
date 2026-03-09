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
```

## Cursor Position

```tests/test_basic.py
def test_static_files(app, client):
    rv = client.get("/static/index.html")
    assert rv.status_code == 200
    assert rv.data.strip() == b"<h1>Hello World!</h1>"
    with app.test_request_context():
        assert flask.url_for("static", filename="index.html") == "/static/index.html"
    rv.close()


de
# ^[CURSOR_POSITION]


def test_static_url_path():
```

## Expected Patch

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_():
#         ^[CURSOR_POSITION]
+    


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_():
#         ^[CURSOR_POSITION]
+    pass


def test_static_url_path():
```


```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_(app, client):
#         ^[CURSOR_POSITION]
+    


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_(app, client):
#         ^[CURSOR_POSITION]
+    pass


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_():
#                ^[CURSOR_POSITION]
+    


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_():
#                ^[CURSOR_POSITION]
+    pass


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_(app, client):
#                ^[CURSOR_POSITION]
+    


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_(app, client):
#                ^[CURSOR_POSITION]
+    pass


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_folder():
#                       ^[CURSOR_POSITION]
+    pass


def test_static_url_path():
```

```diff
--- a/tests/test_basic.py
+++ b/tests/test_basic.py
@@ -1372,15 +1372,15 @@
-de
+def test_static_route_with_host_matching(app, client):
+    
#    ^[CURSOR_POSITION]


def test_static_url_path():
```
