; Outline for Gradle build scripts (Groovy DSL).
;
; Build scripts are structured around configuration closures
; (`dependencies { ... }`, `repositories { ... }`, `android { ... }`),
; property assignments (`group = "..."`), `def`/typed declarations, and - in
; buildSrc / build logic - function and class definitions. We surface those
; so the symbol/breadcrumb navigation works for `.gradle` files.
;
; The declaration patterns follow the Java `outline.scm` convention (the
; pinned tree-sitter-groovy grammar is a Java-family grammar): the body is
; captured as `@item` so members nest underneath it in the outline tree. We
; deliberately do not list every bare method call (e.g. `mavenCentral()`) to
; keep the outline structural rather than one-row-per-statement.
; Configuration closures: `name { ... }` - a call whose argument is a closure.
(juxt_function_call
  name: (identifier) @name
  args: (argument_list
    (closure) @item))

(method_invocation
  name: (identifier) @name
  arguments: (argument_list
    (closure) @item))

; Property assignments, e.g. `group = "com.example"`, `version = "1.0"`.
(assignment_expression
  left: (identifier) @name) @item

(assignment_expression
  left: (field_access
    field: (identifier) @name)) @item

; Function definitions in build logic.
(function_definition
  name: (identifier) @name
  body: (_) @item)

(class_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "sealed"
      "non-sealed"
      "final"
      "strictfp"
      "static"
    ]* @context)?
  "class" @context
  name: (_) @name
  body: (_) @item)

(record_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "sealed"
      "non-sealed"
      "final"
      "strictfp"
      "static"
    ]* @context)?
  "record" @context
  name: (_) @name
  body: (_) @item)

(interface_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "sealed"
      "non-sealed"
      "strictfp"
      "static"
    ]* @context)?
  "interface" @context
  name: (_) @name
  body: (_) @item)

(enum_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "static"
      "final"
      "strictfp"
    ]* @context)?
  "enum" @context
  name: (_) @name
  body: (_) @item)

(annotation_type_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "static"
      "strictfp"
    ]* @context)?
  "@interface" @context
  name: (_) @name
  body: (_) @item)

(enum_constant
  name: (identifier) @name) @item

(method_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "static"
      "final"
      "native"
      "strictfp"
      "synchronized"
    ]* @context)?
  type: (_) @context
  name: (_) @name
  parameters: (formal_parameters
    "(" @context
    (formal_parameter
      type: (_) @context)?
    ("," @context
      (formal_parameter
        type: (_) @context)?)*
    ")" @context)
  body: (_) @item)

(method_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "abstract"
      "static"
      "final"
      "native"
      "strictfp"
      "synchronized"
    ]* @context)?
  type: (_) @context
  name: (_) @name
  parameters: (formal_parameters
    "(" @context
    (formal_parameter
      type: (_) @context)?
    ("," @context
      (formal_parameter
        type: (_) @context)?)*
    ")" @context)
  ";" @item)

(constructor_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "static"
      "final"
    ]* @context)?
  name: (_) @name
  parameters: (formal_parameters
    "(" @context
    (formal_parameter
      type: (_) @context)?
    ("," @context
      (formal_parameter
        type: (_) @context)?)*
    ")" @context)
  body: (_) @item)

(field_declaration
  (modifiers
    [
      "private"
      "public"
      "protected"
      "static"
      "final"
      "transient"
      "volatile"
    ]* @context)?
  type: (_) @context
  declarator: (variable_declarator
    name: (_) @name)) @item

(constant_declaration
  (modifiers
    [
      "public"
      "static"
      "final"
    ]* @context)?
  type: (_) @context
  declarator: (variable_declarator
    name: (_) @name)) @item

(static_initializer
  "static" @context
  (block) @item)
