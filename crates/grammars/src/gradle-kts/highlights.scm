; Highlights for Gradle Kotlin DSL (`.gradle.kts`), written against the
; pinned tree-sitter-kotlin-ng grammar. Adapted from the nvim-treesitter
; Kotlin highlighting structure, which is under the Apache license.
; Identifiers
(identifier) @variable

; `it` keyword inside lambdas
((identifier) @variable.builtin
  (#eq? @variable.builtin "it"))

; `field` keyword inside property getter/setter
((identifier) @variable.builtin
  (#eq? @variable.builtin "field"))

; `this` / `super` keywords inside classes
(this_expression) @variable.builtin

(super_expression) @variable.builtin

(class_parameter
  (identifier) @property)

(class_body
  (property_declaration
    (variable_declaration
      (identifier) @property)))

; obj.property: the member identifier of a navigation expression
(navigation_expression
  (identifier) @property)

(enum_entry
  (identifier) @constant)

; Types
(user_type
  (identifier) @type)

((user_type
  (identifier) @type.builtin)
  (#any-of? @type.builtin
    "Byte" "Short" "Int" "Long" "UByte" "UShort" "UInt" "ULong" "Float" "Double" "Boolean" "Char"
    "String" "Array" "ByteArray" "ShortArray" "IntArray" "LongArray" "UByteArray" "UShortArray"
    "UIntArray" "ULongArray" "FloatArray" "DoubleArray" "BooleanArray" "CharArray" "Map" "Set"
    "List" "EmptyMap" "EmptySet" "EmptyList" "MutableMap" "MutableSet" "MutableList"))

(package_header
  (qualified_identifier) @namespace)

(label) @label

; Function definitions
(function_declaration
  name: (identifier) @function)

(getter
  "get" @function.builtin)

(setter
  "set" @function.builtin)

(primary_constructor) @constructor

(secondary_constructor
  "constructor" @constructor)

(constructor_invocation
  (user_type
    (identifier) @constructor))

(anonymous_initializer
  "init" @constructor)

(parameter
  (identifier) @parameter)

; lambda parameters
(lambda_literal
  (lambda_parameters
    (variable_declaration
      (identifier) @parameter)))

; Function calls
(call_expression
  (identifier) @function)

(call_expression
  (navigation_expression
    (identifier) @function))

((call_expression
  (identifier) @function.builtin)
  (#any-of? @function.builtin
    "arrayOf" "arrayOfNulls" "byteArrayOf" "shortArrayOf" "intArrayOf" "longArrayOf" "ubyteArrayOf"
    "ushortArrayOf" "uintArrayOf" "ulongArrayOf" "floatArrayOf" "doubleArrayOf" "booleanArrayOf"
    "charArrayOf" "emptyArray" "mapOf" "setOf" "listOf" "emptyMap" "emptySet" "emptyList"
    "mutableMapOf" "mutableSetOf" "mutableListOf" "print" "println" "error" "TODO" "run"
    "runCatching" "repeat" "lazy" "lazyOf" "enumValues" "enumValueOf" "assert" "check"
    "checkNotNull" "require" "requireNotNull" "with" "suspend" "synchronized"))

; Literals
[
  (line_comment)
  (block_comment)
  (shebang)
] @comment

(float_literal) @float

(number_literal) @number

; The kotlin-ng grammar has no dedicated boolean/null literal nodes; they
; lex as identifiers, so match them by name.
((identifier) @boolean
  (#any-of? @boolean "true" "false"))

((identifier) @constant.builtin
  (#eq? @constant.builtin "null"))

(character_literal) @character

[
  (string_literal)
  (multiline_string_literal)
] @string

(escape_sequence) @string.escape

(interpolation
  "${" @punctuation.special)

; Keywords
(type_alias
  "typealias" @keyword)

[
  (class_modifier)
  (member_modifier)
  (function_modifier)
  (property_modifier)
  (platform_modifier)
  (variance_modifier)
  (parameter_modifier)
  (visibility_modifier)
  (reification_modifier)
  (inheritance_modifier)
] @keyword

[
  "if"
  "else"
  "when"
  "for"
  "while"
  "do"
  "try"
  "catch"
  "throw"
  "finally"
  "val"
  "var"
  "enum"
  "class"
  "object"
  "interface"
  "companion"
  "package"
  "import"
] @keyword

"fun" @keyword.function

(return_expression) @keyword.return

(annotation
  "@" @attribute
  (use_site_target)? @attribute)

(annotation
  (user_type
    (identifier) @attribute))

(annotation
  (constructor_invocation
    (user_type
      (identifier) @attribute)))

(file_annotation
  "@" @attribute
  "file" @attribute
  ":" @attribute)

(file_annotation
  (user_type
    (identifier) @attribute))

(file_annotation
  (constructor_invocation
    (user_type
      (identifier) @attribute)))

; Operators & Punctuation
[
  "!"
  "!="
  "!=="
  "="
  "=="
  "==="
  ">"
  ">="
  "<"
  "<="
  "||"
  "&&"
  "+"
  "++"
  "+="
  "-"
  "--"
  "-="
  "*"
  "*="
  "/"
  "/="
  "%"
  "%="
  "?."
  "?:"
  "!!"
  "is"
  "!is"
  "in"
  "!in"
  "as"
  "as?"
  ".."
  "..<"
  "->"
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "."
  ","
  ";"
  ":"
  "::"
] @punctuation.delimiter
