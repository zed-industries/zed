# Rules for Writing Optimization Rules

For both correctness and compile speed, we must be careful with our rules. A lot
of it boils down to the fact that, unlike traditional e-graphs, our rules are
*directional*.

1. Rules should not rewrite to worse code: the right-hand side should be at
   least as good as the left-hand side or better.

   For example, the rule

       x => (add x 0)

   is disallowed, but swapping its left- and right-hand sides produces a rule
   that is allowed.

   Any kind of canonicalizing rule that intends to help subsequent rules match
   and unlock further optimizations (e.g. floating constants to the right side
   for our constant-propagation rules to match) must produce canonicalized
   output that is no worse than its noncanonical input.

   We assume this invariant as a heuristic to break ties between two
   otherwise-equal-cost expressions in various places, making up for some
   limitations of our explicit cost function.

2. Any rule that removes value-uses in its right-hand side that previously
   existed in its left-hand side MUST use `subsume`.

   For example, the rule

       (select 1 x y) => x

   MUST use `subsume`.

   This is required for correctness because, once a value-use is removed, some
   e-nodes in the e-class are more equal than others. There might be uses of `x`
   in a scope where `y` is not available, and so emitting `(select 1 x y)` in
   place of `x` in such cases would introduce uses of `y` where it is not
   defined.

   An exception to this rule is discarding constants, as they can be
   rematerialized anywhere without introducing correctness issues. For example,
   the (admittedly silly) rule `(select 1 x (iconst_u _)) => x` would be a good
   candidate for not using `subsume`, as it does not discard any non-constant
   values introduced in its LHS.

3. Avoid overly general rewrites like commutativity and associativity. Instead,
   prefer targeted instances of the rewrite (for example, canonicalizing adds
   where one operand is a constant such that the constant is always the add's
   second operand, rather than general commutativity for adds) or even writing
   the "same" optimization rule multiple times.

   For example, the commutativity in the first rule in the following snippet is
   bad because it will match even when the first operand is not an add:

       ;; Commute to allow `(foo (add ...) x)`, when we see it, to match.
       (foo x y) => (foo y x)

       ;; Optimize.
       (foo x (add ...)) => (bar x)

   Better is to commute only when we know that canonicalizing in this way will
   all definitely allow the subsequent optimization rule to match:

       ;; Canonicalize all adds to `foo`'s second operand.
       (foo (add ...) x) => (foo x (add ...))

       ;; Optimize.
       (foo x (add ...)) => (bar x)

   But even better in this case is to write the "same" optimization multiple
   times:

       (foo (add ...) x) => (bar x)
       (foo x (add ...)) => (bar x)

   The cost of rule-matching is amortized by the ISLE compiler, where as the
   intermediate result of each rewrite allocates new e-nodes and requires
   storage in the dataflow graph. Therefore, additional rules are cheaper than
   additional e-nodes.

   Commutativity and associativity in particular can cause huge amounts of
   e-graph bloat.

   One day we intend to extend ISLE with built-in support for commutativity, so
   we don't need to author the redundant commutations ourselves:
   https://github.com/bytecodealliance/wasmtime/issues/6128

4. Be careful with (ideally avoid) multiple matches on the same `Value`, as
   they can result in surprising multi-matching behavior. Be skeptical of
   helpers that can inadvertently create this behavior.

   In our mid-end ISLE environment, a `Value` corresponds to an eclass, with
   multiple possible representations. A rule that matches on a `Value` will
   traverse all enodes in the eclass, looking for a match. This is usually
   exactly what we want: it is what allows a pattern like `(iadd (iconst k) x)`
   to find the `iconst` amongst multiple possibilities for the argument.

   However, this can also result in surprising behavior. If one has a helper
   and a simplify rule like

       (decl suitable_for_rewrite (Value) Value)
       (rule (suitable_for_rewrite x @ (iadd ...)) x)
       (rule (suitable_for_rewrite x @ (isub ...)) x)

       (rule (simplify (ireduce _ x))
         (if-let _ (suitable_for_rewrite x))
         x)

    Then this can result in the extremely surprising behavior that `(ireduce
    (other_op ...))` matches, if `(other_op ...)` is in the same eclass as an
    `iadd` or `isub`. This happens because the left-hand side binds `x`, which
    describes the entire eclass; and `suitable_for_rewrite` matches if *any*
    representation of `x` matches.

    This resulted in a real bug in #7999. The best guidance is to keep rules
    simple and direct: rather than attempting to abstract out helpers and
    perform multiple, separate, matches on a `Value`, write patterns directly.
    This has the additional benefit that the rewrites are more clearly visible
    to the casual reader. For example:

        (rule (simplify (ireduce _ (iadd ...)))
              (iadd ...))
        (rule (simplify (ireduce _ (isub ...)))
              (isub ...))
