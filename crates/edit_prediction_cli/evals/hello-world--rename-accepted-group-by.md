+++
repository_url = "https://github.com/octocat/hello-world"
revision = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d"
+++

## Edit History

```diff
--- a/README
+++ b/README
@@ -1,1 +1,6 @@
-Hello World!
+function filterByStatus(items, status) {
+    return items.filter(item => item.status === status);
+}
+
+function groupBy
+
```

// User accepted prediction:
```diff
--- a/README
+++ b/README
@@ -4,3 +4,9 @@
 
-function groupBy
+function groupByStatus(items) {
+    return items.reduce((groups, item) => {
+        const key = item.status;
+        (groups[key] = groups[key] || []).push(item);
+        return groups;
+    }, {});
+}
 
```

```diff
--- a/README
+++ b/README
@@ -4,4 +4,4 @@
 
-function groupByStatus(items) {
+function groupByCat(items) {
     return items.reduce((groups, item) => {
```

## Cursor Position

```README
function filterByStatus(items, status) {
    return items.filter(item => item.status === status);
}

function groupByCat(items) {
#                  ^[CURSOR_POSITION]
    return items.reduce((groups, item) => {
        const key = item.status;
        (groups[key] = groups[key] || []).push(item);
        return groups;
    }, {});
}

```

## Expected Patch

```diff
--- a/README
+++ b/README
@@ -5,7 +5,7 @@
-function groupByCat(items) {
+function groupByCategory(items) {
#                        ^[CURSOR_POSITION]
     return items.reduce((groups, item) => {
-        const key = item.status;
+        const key = item.category;
         (groups[key] = groups[key] || []).push(item);
         return groups;
     }, {});
```
