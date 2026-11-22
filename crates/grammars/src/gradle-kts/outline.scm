(package_header
  "package" @context
  (qualified_identifier) @name) @item

(class_declaration
  (modifiers)? @context
  name: (identifier) @name) @item

(object_declaration
  "object" @context
  name: (identifier) @name) @item

(type_alias
  "typealias" @context
  type: (identifier) @name) @item

(enum_entry
  (identifier) @name) @item

(function_declaration
  "fun" @context
  name: (identifier) @name) @item

(property_declaration
  (variable_declaration
    (identifier) @name)) @item

(property_declaration
  (multi_variable_declaration
    (variable_declaration
      (identifier) @name) @item))

(secondary_constructor
  "constructor" @name) @item

(anonymous_initializer
  "init" @name) @item

; --- Gradle build-script DSL -------------------------------------------------
; The declarations above cover generic Kotlin, but a `build.gradle.kts` is mostly
; configuration blocks and assignments. Surface those so the outline reflects the
; build structure rather than just the rare top-level `val`/`fun`/`class`.
; Configuration blocks: `name { ... }` - a call with a trailing lambda, e.g.
; `plugins { ... }`, `dependencies { ... }`, `repositories { ... }`, `doLast { ... }`.
; The lambda body is captured as `@item` so members nest underneath.
(call_expression
  (identifier) @name
  (annotated_lambda
    (lambda_literal) @item))

; Top-level property assignments, e.g. `group = "com.example"`, `version = "..."`.
(assignment
  left: (identifier) @name) @item
