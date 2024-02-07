(adt
  "data" @context
  name: (type) @name) @item

(type_alias
  "type" @context
  name: (type) @name) @item

(newtype
  "newtype" @context
  name: (type) @name) @item

(signature
  name: (variable) @name) @item

(class
  "class" @context
  (class_head) @name) @item

(instance
  "instance" @context
  (instance_head) @name) @item

(foreign_import
  "foreign" @context
  (impent) @name) @item
