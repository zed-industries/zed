+++
repository_url = "https://github.com/hashicorp/terraform"
revision = "a3dc571150a7651a1a4a8b302342d26089c97795"
+++

## Edit History

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
@@ -63,6 +63,7 @@
 	a.mu.Lock()
 	defer a.mu.Unlock()

+	/
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
```

## Cursor Position

```internal/actions/actions.go
	defer a.mu.Unlock()

	data, ok := a.actionInstances.GetOk(addr)

	if !ok {
		return nil, false
	}

	return &data, true
}

func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
	a.mu.Lock()
	defer a.mu.Unlock()

	/
   // <[CURSOR_POSITION]
	result := []addrs.AbsActionInstance{}
	for _, data := range a.actionInstances.Elements() {
		if data.Key.ContainingAction().Equal(addr) {
			result = append(result, data.Key)
		}
	}

	return result
}
```

## Expected Patch

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
@@ -51,26 +51,26 @@
 func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
 	a.mu.Lock()
 	defer a.mu.Unlock()

-	/
+	// Filter action instances by the given action.
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
 			result = append(result, data.Key)
 		}
 	}
```

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
@@ -54,25 +54,25 @@
 func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
 	a.mu.Lock()
 	defer a.mu.Unlock()

-	/
+	// Filter action instances that belong to the given action
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
 			result = append(result, data.Key)
 		}
 	}
```

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
@@ -54,25 +54,25 @@
 func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
 	a.mu.Lock()
 	defer a.mu.Unlock()

-	/
+	// Iterate through all action instances and filter by the containing action
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
 			result = append(result, data.Key)
 		}
 	}
```

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
 func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
 	a.mu.Lock()
 	defer a.mu.Unlock()

-	/
+	// Iterate through all action instances and return those that belong to the given action
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
 			result = append(result, data.Key)
 		}
 	}
```

```diff
--- a/internal/actions/actions.go
+++ b/internal/actions/actions.go
 func (a *Actions) GetActionInstanceKeys(addr addrs.AbsAction) []addrs.AbsActionInstance {
 	a.mu.Lock()
 	defer a.mu.Unlock()

-	/
+	// Collect all action instances that belong to the given action
 	result := []addrs.AbsActionInstance{}
 	for _, data := range a.actionInstances.Elements() {
 		if data.Key.ContainingAction().Equal(addr) {
 			result = append(result, data.Key)
 		}
 	}
```
