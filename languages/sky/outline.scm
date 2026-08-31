; outline.scm — Code outline/structure for Sky

(module_declaration
  (module_name) @name) @item

(function_declaration
  name: (lower_identifier) @name) @item

(function_declaration
  name: (upper_identifier) @name) @item

(type_declaration
  name: (upper_identifier) @name) @item

(type_alias_declaration
  name: (upper_identifier) @name) @item

(type_annotation_declaration
  name: (lower_identifier) @name) @item

(port_declaration
  (lower_identifier) @name) @item

(import_declaration
  (module_name) @name) @item