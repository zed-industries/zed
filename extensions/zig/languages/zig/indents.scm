[
  (AsmExpr)
  (AssignExpr)
  (Block)
  (BlockExpr)
  (ContainerDecl)
  (ErrorUnionExpr)
  (InitList)
  (SwitchExpr)
  (TestDecl)
] @indent.begin

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

[
  (line_comment)
  (container_doc_comment)
  (doc_comment)
  (LINESTRING)
] @indent.ignore
