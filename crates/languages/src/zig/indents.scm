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

[
  "}"
  "]"
  ")"
] @indent.branch

[
  (line_comment)
  (container_doc_comment)
  (doc_comment)
  (LINESTRING)
] @indent.ignore
