/// Define a normal symbol, as no-argument commands like \alpha
///
/// Arguments:
/// - s (str): Alias command for typst handler.
///   For example, alias `\prod` to typst's `product`.
/// - sym (content): The specific content, as the value of alias in mitex-scope.
///   For example, there is no direct alias for \negthinspace symbol in typst,
///   but we can add `h(-(3/18) * 1em)` ourselves
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-sym(s, sym: none) = {
  (
    (kind: "alias-sym", alias: s),
    if sym != none {
      (alias: s, handle: sym)
    } else {
      none
    },
  )
}

/// Define a greedy command, like \displaystyle
///
/// Arguments:
/// - s (str): Alias command for typst handler.
///   For example, alias `\displaystyle` to typst's `mitexdisplay`, as the key in mitex-scope.
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   It receives a content argument as all greedy matches to the content
///   For example, we define `mitexdisplay` to `math.display`
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-greedy-cmd(s, handle: none) = {
  (
    (kind: "greedy-cmd", alias: s),
    if handle != none {
      (alias: s, handle: handle)
    } else {
      none
    },
  )
}

/// Define an infix command, like \over
///
/// Arguments:
/// - s (str): Alias command for typst handler.
///   For example, alias `\over` to typst's `frac`, as the key in mitex-scope.
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   It receives two content arguments, as (prev, after) arguments.
///   For example, we define `\over` to `frac: (num, den) => $(num)/(den)$`
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-infix-cmd(s, handle: none) = {
  (
    (kind: "infix-cmd", alias: s),
    if handle != none {
      (alias: s, handle: handle)
    } else {
      none
    },
  )
}

/// Define a glob (Global Wildcard) match command with a specified pattern for matching args
/// Kind of item to match:
/// - Bracket/b: []
/// - Parenthesis/p: ()
/// - Term/t: any rest of terms, typically {} or single char
///
/// Arguments:
/// - pat (pattern): The pattern for glob-cmd
///   For example, `{,b}t` for `\sqrt` to support `\sqrt{2}` and `\sqrt[3]{2}`
/// - s (str): Alias command for typst handler.
///   For example, alias `\sqrt` to typst's `mitexsqrt`, as the key in mitex-scope.
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   It receives variable length arguments, for example `(2,)` or `([3], 2)` for sqrt.
///   Therefore you need to use `(.. arg) = > {..}` to receive them.
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-glob-cmd(pat, s, handle: none) = {
  (
    (kind: "glob-cmd", pattern: pat, alias: s),
    if handle != none {
      (alias: s, handle: handle)
    } else {
      none
    },
  )
}

/// Define a command with a fixed number of arguments, like \hat{x} and \frac{1}{2}
///
/// Arguments:
/// - num (int): The number of arguments for the command.
/// - alias (str): Alias command for typst handler.
///   For example, alias `\frac` to typst's `frac`, as the key in mitex-scope.
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   It receives fixed number of arguments, for example `frac(1, 2)` for `\frac{1}{2}`.
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-cmd(num, alias: none, handle: none) = {
  (
    (
      kind: "cmd",
      args: ("kind": "right", "pattern": (kind: "fixed-len", len: num)),
      alias: alias,
    ),
    if handle != none {
      (alias: alias, handle: handle)
    } else {
      none
    },
  )
}

/// Define an environment with a fixed number of arguments, like \begin{alignedat}{2}
///
/// Arguments:
/// - num (int): The number of arguments as environment options for the environment.
/// - alias (str): Alias command for typst handler.
///   For example, alias `\begin{alignedat}{2}` to typst's `alignedat`,
///   and alias `\begin{aligned}` to typst's `aligned`, as the key in mitex-scope.
/// - kind (str): environment kind, it could be "is-math", "is-cases", "is-matrix",
///   "is-itemize", "is-enumerate"
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   It receives fixed number of named arguments as environment options,
///   for example `alignedat(arg0: ..)` or `alignedat(arg0: .., arg1: ..)`.
///   And it receives variable length arguments as environment body,
///   Therefore you need to use `(.. arg) = > {..}` to receive them.
///
/// Return: A spec item and a scope item (none for no scope item)
#let define-env(num, kind: "none", alias: none, handle: none) = {
  (
    (
      kind: "env",
      args: if num != none {
        (kind: "fixed-len", len: num)
      } else {
        (kind: "none")
      },
      ctx_feature: (kind: kind),
      alias: alias,
    ),
    if handle != none {
      (alias: alias, handle: handle)
    } else {
      none
    },
  )
}

#let define-glob-env(pat, kind: "none", alias: none, handle: none) = {
  (
    (
      kind: "glob-env",
      pattern: pat,
      ctx_feature: (kind: kind),
      alias: alias,
    ),
    if handle != none {
      (alias: alias, handle: handle)
    } else {
      none
    },
  )
}

/// Define a symbol without alias and without handler function, like \alpha => alpha
///
/// Return: A spec item and no scope item (none for no scope item)
#let sym = ((kind: "sym"), none)

/// Define a symbol without alias and with handler function,
/// like \negthinspace => h(-(3/18) * 1em)
///
/// Arguments:
/// - handle (function): The handler function, as the value of alias in mitex-scope.
///   For example, define `negthinspace` to handle `h(-(3/18) * 1em)` in mitex-scope
///
/// Return: A symbol spec and a scope item
#let of-sym(handle) = ((kind: "sym"), (handle: handle))

/// Define a left1-op command without handler, like `\limits` for `\sum\limits`
///
/// Arguments:
/// - alias (str): Alias command for typst handler.
///   For example, alias `\limits` to typst's `limits`
///   and alias `\nolimits` to typst's `scripts`
///
/// Return: A cmd spec and no scope item (none for no scope item)
#let left1-op(alias) = ((kind: "cmd", args: (kind: "left1"), alias: alias), none)

/// Define a cmd1 command like \hat{x} => hat(x)
///
/// Return: A cmd1 spec and a scope item (none for no scope item)
#let cmd1 = ((kind: "cmd1"), none)

/// Define a cmd2 command like \binom{1}{2} => binom(1, 2)
///
/// Return: A cmd2 spec and a scope item (none for no scope item)
#let cmd2 = ((kind: "cmd2"), none)

/// Define a matrix environment without handler
///
/// Return: A matrix-env spec and a scope item (none for no scope item)
#let matrix-env = ((kind: "matrix-env"), none)

/// Receives a list of definitions composed of the above functions, and processes them to return a dictionary containing spec and scope.
#let process-spec(definitions) = {
  let spec = (:)
  let scope = (:)
  for (key, value) in definitions.pairs() {
    let spec-item = value.at(0)
    let scope-item = value.at(1)
    spec.insert(key, spec-item)
    if scope-item != none {
      if "alias" in scope-item and type(scope-item.alias) == str {
        let key = if scope-item.alias.starts-with("#") {
          scope-item.alias.slice(1)
        } else {
          scope-item.alias
        }
        scope.insert(key, scope-item.handle)
      } else {
        scope.insert(key, scope-item.handle)
      }
    }
  }
  (spec: spec, scope: scope)
}
