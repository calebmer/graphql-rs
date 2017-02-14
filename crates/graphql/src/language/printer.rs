use super::ast;

/// Prints a GraphQL AST to a string using some reasonable default formatting
/// options.
pub fn print(document: &ast::Document) -> String {
  let mut printer = Printer {
    output: String::new(),
    indentation_size: 2,
    indentation_level: 0,
  };
  printer.print_document(document);
  printer.output
}

/// A printer instance holds some internal printing state including the output
/// string and the indentation level.
struct Printer {
  /// The output string. This is the output which can be returned when we are
  /// done printing.
  output: String,
  /// The size of a single level of indentation.
  indentation_size: u8,
  /// State for the current indentation level of our printer.
  indentation_level: u8,
}

impl Printer {
  //////////////////////////////////////////////////////////////////////////////
  // Utilities
  //////////////////////////////////////////////////////////////////////////////

  /// Adds a character to the internal output we are printing.
  fn push_char(&mut self, character: char) {
    self.output.push(character);
  }

  /// Adds a string to the internal output we are printing.
  fn push(&mut self, string: &str) {
    self.output.push_str(string);
  }

  /// Prints many itmes using a given printer function and a separator string.
  /// The separator string will be printed in between each item. Works somewhat
  /// like a join.
  fn many<T>(
    &mut self,
    items: &Vec<T>,
    print_fn: fn(&mut Printer, &T) -> (),
    separator: &str,
  ) {
    let mut iter = items.iter();
    if let Some(item) = iter.next() {
      print_fn(self, item);
    }
    for item in iter {
      self.push(separator);
      print_fn(self, item);
    }
  }

  /// Increases the printer state for the internal indentation level. Now
  /// whenever `line` is called an extra level of indentation will be added.
  fn indent(&mut self) {
    self.indentation_level += self.indentation_size;
  }

  /// Removes a level of indentation from the internal printer state. Now
  /// whenever `line` is called a level of indentation will be removed.
  ///
  /// Will panic if the indentation level is currently 0.
  fn deindent(&mut self) {
    self.indentation_level -= self.indentation_size;
  }

  /// Creates a new line with the internal printer indentation state.
  fn line(&mut self) {
    self.push("\n");
    for _ in 0..self.indentation_level {
      self.push(" ");
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Name
  //////////////////////////////////////////////////////////////////////////////

  fn print_name(&mut self, name: &ast::Name) {
    self.push(&name.value);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Document
  //////////////////////////////////////////////////////////////////////////////

  fn print_document(&mut self, node: &ast::Document) {
    self.many(
      &node.definitions,
      Printer::print_definition,
      "\n\n",
    );
    self.push("\n");
  }

  fn print_definition(&mut self, node: &ast::Definition) {
    match node {
      &ast::Definition::Operation(ref operation) => self.print_operation_definition(operation),
      &ast::Definition::Fragment(ref fragment) => self.print_fragment_definition(fragment),

      // The type system AST extension is an experimental non-spec addition.
      #[cfg(feature = "type_system")]
      &ast::Definition::TypeSystem(ref type_system) => self.print_type_system_definition(type_system),
    }
  }

  fn print_operation_definition(&mut self, node: &ast::OperationDefinition) {
    // Anonymous queries with no directives or variable definitions can use the
    // query short form which is just the selection set.
    if {
      node.operation == ast::OperationType::Query &&
      node.name.is_none() &&
      node.variable_definitions.is_empty() &&
      node.directives.is_empty()
    } {
      self.print_selection_set(&node.selection_set);
      return;
    }
    self.print_operation_type(&node.operation);
    self.push(" ");
    if let Some(ref name) = node.name {
      self.print_name(name);
      // If we are not going to print variable definitions then let us add a
      // space now because a space will not be added after we print the
      // definitions.
      if node.variable_definitions.is_empty() {
        self.push(" ");
      }
    }
    if !node.variable_definitions.is_empty() {
      self.push("(");
      self.many(
        &node.variable_definitions,
        Printer::print_variable_definition,
        ", ",
      );
      self.push(") ");
    }
    self.many(
      &node.directives,
      Printer::print_directive,
      " ",
    );
    if !node.directives.is_empty() {
      self.push(" ");
    }
    self.print_selection_set(&node.selection_set);
  }

  fn print_operation_type(&mut self, operation: &ast::OperationType) {
    self.push(match operation {
      &ast::OperationType::Query => "query",
      &ast::OperationType::Mutation => "mutation",

      // Subscriptions are an expiremental non-spec addition.
      #[cfg(feature = "subscriptions")]
      &ast::OperationType::Subscription => "subscription",
    });
  }

  fn print_variable_definition(&mut self, node: &ast::VariableDefinition) {
    self.print_variable(&node.variable);
    self.push(": ");
    self.print_type_reference(&node.typ);
    if let Some(ref default_value) = node.default_value {
      self.push(" = ");
      self.print_value_literal(default_value);
    }
  }

  fn print_variable(&mut self, node: &ast::Variable) {
    self.push("$");
    self.print_name(&node.name);
  }

  fn print_selection_set(&mut self, node: &ast::SelectionSet) {
    self.push("{");
    self.indent();
    for selection in &node.selections {
      self.line();
      self.print_selection(&selection);
    }
    self.deindent();
    self.line();
    self.push("}");
  }

  fn print_selection(&mut self, node: &ast::Selection) {
    match node {
      &ast::Selection::Field(ref field) => self.print_field(field),
      &ast::Selection::FragmentSpread(ref fragment) => self.print_fragment_spread(fragment),
      &ast::Selection::InlineFragment(ref fragment) => self.print_inline_fragment(fragment),
    }
  }

  fn print_field(&mut self, node: &ast::Field) {
    if let Some(ref alias) = node.alias {
      self.print_name(alias);
      self.push(": ");
    }
    self.print_name(&node.name);
    if !node.arguments.is_empty() {
      self.push("(");
      self.many(
        &node.arguments,
        Printer::print_argument,
        ", ",
      );
      self.push(")");
    }
    self.print_directives(&node.directives);
    if let Some(ref selection_set) = node.selection_set {
      self.push(" ");
      self.print_selection_set(selection_set);
    }
  }

  fn print_argument(&mut self, node: &ast::Argument) {
    self.print_name(&node.name);
    self.push(": ");
    self.print_value_literal(&node.value);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Fragments
  //////////////////////////////////////////////////////////////////////////////

  fn print_fragment_spread(&mut self, node: &ast::FragmentSpread) {
    self.push("...");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
  }

  fn print_inline_fragment(&mut self, node: &ast::InlineFragment) {
    self.push("...");
    if let Some(ref type_condition) = node.type_condition {
      self.push(" on ");
      self.print_named_type(type_condition);
    }
    self.print_directives(&node.directives);
    self.push(" ");
    self.print_selection_set(&node.selection_set);
  }

  fn print_fragment_definition(&mut self, node: &ast::FragmentDefinition) {
    self.push("fragment ");
    self.print_name(&node.name);
    self.push(" on ");
    self.print_named_type(&node.type_condition);
    self.print_directives(&node.directives);
    self.push(" ");
    self.print_selection_set(&node.selection_set);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Values
  //////////////////////////////////////////////////////////////////////////////

  fn print_value_literal(&mut self, node: &ast::Value) {
    match node {
      &ast::Value::Variable(ref node) => self.print_variable(node),
      &ast::Value::Int(ref node) => self.push(&node.value.to_string()),
      &ast::Value::Float(ref node) => self.push(&node.value.to_string()),
      &ast::Value::String(ref node) => self.print_string_value(node),
      &ast::Value::Boolean(ref node) => self.print_boolean_value(node),
      &ast::Value::Null(_) => self.push("null"),
      &ast::Value::Enum(ref node) => self.push(&node.value),
      &ast::Value::List(ref node) => self.print_list_value(node),
      &ast::Value::Object(ref node) => self.print_object_value(node),
    }
  }

  fn print_string_value(&mut self, node: &ast::StringValue) {
    self.push("\"");
    for character in node.value.chars() {
      match character {
        '"' => self.push("\\\""),
        '\\' => self.push("\\\\"),
        '\n' => self.push("\\n"),
        character => self.push_char(character),
      }
    }
    self.push("\"");
  }

  fn print_boolean_value(&mut self, node: &ast::BooleanValue) {
    if node.value {
      self.push("true");
    } else {
      self.push("false");
    }
  }

  fn print_list_value(&mut self, node: &ast::ListValue) {
    self.push("[");
    self.many(
      &node.values,
      Printer::print_value_literal,
      ", ",
    );
    self.push("]");
  }

  fn print_object_value(&mut self, node: &ast::ObjectValue) {
    self.push("{");
    self.many(
      &node.fields,
      Printer::print_object_field,
      ", ",
    );
    self.push("}");
  }

  fn print_object_field(&mut self, node: &ast::ObjectField) {
    self.print_name(&node.name);
    self.push(": ");
    self.print_value_literal(&node.value);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Directives
  //////////////////////////////////////////////////////////////////////////////

  fn print_directives(&mut self, directives: &Vec<ast::Directive>) {
    if !directives.is_empty() {
      self.push(" ");
      self.many(
        directives,
        Printer::print_directive,
        " ",
      );
    }
  }

  fn print_directive(&mut self, node: &ast::Directive) {
    self.push("@");
    self.print_name(&node.name);
    if !node.arguments.is_empty() {
      self.push("(");
      self.many(
        &node.arguments,
        Printer::print_argument,
        ", ",
      );
      self.push(")");
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Type Reference
  //////////////////////////////////////////////////////////////////////////////

  fn print_type_reference(&mut self, node: &ast::Type) {
    match node {
      &ast::Type::Named(ref node) => self.print_named_type(node),
      &ast::Type::List(ref node) => self.print_list_type(node),
      &ast::Type::NonNull(ref node) => self.print_non_null_type(node),
    }
  }

  fn print_named_type(&mut self, node: &ast::NamedType) {
    self.print_name(&node.name);
  }

  fn print_list_type(&mut self, node: &ast::ListType) {
    self.push("[");
    self.print_type_reference(&node.typ);
    self.push("]");
  }

  fn print_non_null_type(&mut self, node: &ast::NonNullType) {
    match *node.typ {
      ast::NullableType::Named(ref node) => self.print_named_type(node),
      ast::NullableType::List(ref node) => self.print_list_type(node),
    }
    self.push("!");
  }
}

////////////////////////////////////////////////////////////////////////////////
// Type System Definition
////////////////////////////////////////////////////////////////////////////////
//
// The type system AST extension is an experimental non-spec addition.
#[cfg(feature = "type_system")]
impl Printer {
  fn print_type_system_definition(&mut self, node: &ast::TypeSystemDefinition) {
    use self::ast::TypeSystemDefinition::*;
    use self::ast::TypeDefinition::*;
    match node {
      &Schema(ref node) => self.print_schema_definition(node),
      &Type(Scalar(ref node)) => self.print_scalar_type_definition(node),
      &Type(Object(ref node)) => self.print_object_type_definition(node),
      &Type(Interface(ref node)) => self.print_interface_type_definition(node),
      &Type(Union(ref node)) => self.print_union_type_definition(node),
      &Type(Enum(ref node)) => self.print_enum_type_definition(node),
      &Type(InputObject(ref node)) => self.print_input_object_type_definition(node),
      &TypeExtension(ref node) => self.print_type_extension_definition(node),
      &Directive(ref node) => self.print_directive_definition(node),
    }
  }

  fn print_schema_definition(&mut self, node: &ast::SchemaDefinition) {
    self.push("schema ");
    self.print_directives(&node.directives);
    self.push("{");
    self.indent();
    for operation_type in &node.operation_types {
      self.line();
      self.print_operation_type_definition(&operation_type);
    }
    self.deindent();
    self.line();
    self.push("}");
  }

  fn print_operation_type_definition(&mut self, node: &ast::OperationTypeDefinition) {
    self.print_operation_type(&node.operation);
    self.push(": ");
    self.print_named_type(&node.typ);
  }

  fn print_scalar_type_definition(&mut self, node: &ast::ScalarTypeDefinition) {
    self.push("scalar ");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
  }

  fn print_object_type_definition(&mut self, node: &ast::ObjectTypeDefinition) {
    self.push("type ");
    self.print_name(&node.name);
    if !node.interfaces.is_empty() {
      self.push(" implements ");
      self.many(
        &node.interfaces,
        Printer::print_named_type,
        ", ",
      );
    }
    self.print_directives(&node.directives);
    self.push(" {");
    if !node.fields.is_empty() {
      self.indent();
      for field in &node.fields {
        self.line();
        self.print_field_definition(&field);
      }
      self.deindent();
      self.line();
    }
    self.push("}");
  }

  fn print_field_definition(&mut self, node: &ast::FieldDefinition) {
    self.print_name(&node.name);
    if !node.arguments.is_empty() {
      self.push("(");
      self.many(
        &node.arguments,
        Printer::print_input_value_definition,
        ", ",
      );
      self.push(")");
    }
    self.push(": ");
    self.print_type_reference(&node.typ);
    self.print_directives(&node.directives);
  }

  fn print_input_value_definition(&mut self, node: &ast::InputValueDefinition) {
    self.print_name(&node.name);
    self.push(": ");
    self.print_type_reference(&node.typ);
    if let Some(ref default_value) = node.default_value {
      self.push(" = ");
      self.print_value_literal(default_value);
    }
    self.print_directives(&node.directives);
  }

  fn print_interface_type_definition(&mut self, node: &ast::InterfaceTypeDefinition) {
    self.push("interface ");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
    self.push(" {");
    self.indent();
    for field in &node.fields {
      self.line();
      self.print_field_definition(&field);
    }
    self.deindent();
    self.line();
    self.push("}");
  }

  fn print_union_type_definition(&mut self, node: &ast::UnionTypeDefinition) {
    self.push("union ");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
    self.push(" = ");
    self.many(
      &node.types,
      Printer::print_named_type,
      " | ",
    );
  }

  fn print_enum_type_definition(&mut self, node: &ast::EnumTypeDefinition) {
    self.push("enum ");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
    self.push(" {");
    self.indent();
    for value in &node.values {
      self.line();
      self.print_enum_value_definition(&value);
    }
    self.deindent();
    self.line();
    self.push("}");
  }

  fn print_enum_value_definition(&mut self, node: &ast::EnumValueDefinition) {
    self.print_name(&node.name);
    self.print_directives(&node.directives);
  }

  fn print_input_object_type_definition(&mut self, node: &ast::InputObjectTypeDefinition) {
    self.push("input ");
    self.print_name(&node.name);
    self.print_directives(&node.directives);
    self.push(" {");
    self.indent();
    for field in &node.fields {
      self.line();
      self.print_input_value_definition(&field);
    }
    self.deindent();
    self.line();
    self.push("}");
  }

  fn print_type_extension_definition(&mut self, node: &ast::TypeExtensionDefinition) {
    self.push("extend ");
    self.print_object_type_definition(&node.definition);
  }

  fn print_directive_definition(&mut self, node: &ast::DirectiveDefinition) {
    self.push("directive @");
    self.print_name(&node.name);
    if !node.arguments.is_empty() {
      self.push("(");
      self.many(
        &node.arguments,
        Printer::print_input_value_definition,
        ", ",
      );
      self.push(")");
    }
    self.push(" on ");
    self.many(
      &node.locations,
      Printer::print_name,
      " | ",
    );
  }
}
