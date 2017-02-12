use super::ast;

pub fn print(document: &ast::Document) -> String {
  let mut printer = Printer {
    string: String::new(),
    indentation_size: 2,
    indentation_level: 0,
  };
  printer.print_document(document);
  printer.string
}

struct Printer {
  string: String,
  indentation_size: u8,
  indentation_level: u8,
}

impl Printer {
  //////////////////////////////////////////////////////////////////////////////
  // Utilities
  //////////////////////////////////////////////////////////////////////////////

  fn push(&mut self, character: char) {
    self.string.push(character);
  }

  fn push_str(&mut self, string: &str) {
    self.string.push_str(string);
  }

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
      self.push_str(separator);
      print_fn(self, item);
    }
  }

  fn indent(&mut self) {
    self.indentation_level += self.indentation_size;
  }

  fn deindent(&mut self) {
    self.indentation_level -= self.indentation_size;
  }

  fn line(&mut self) {
    self.push('\n');
    for _ in 0..self.indentation_level {
      self.push(' ');
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Name
  //////////////////////////////////////////////////////////////////////////////

  fn print_name(&mut self, name: &ast::Name) {
    self.push_str(&name.value);
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
    self.push('\n');
  }

  fn print_definition(&mut self, node: &ast::Definition) {
    match node {
      &ast::Definition::Operation(ref operation) => self.print_operation_definition(operation),
      &ast::Definition::Fragment(ref fragment) => self.print_fragment_definition(fragment),

      // // The type system AST extension is an experimental non-spec addition.
      // #[cfg(feature = "type_system")]
      // &ast::Definition::TypeSystem(ref type_system) => self.print_type_system_definition(type_system),
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
    self.push(' ');

    if let Some(ref name) = node.name {
      self.print_name(name);

      // If we are not going to print variable definitions then let us add a
      // space now because a space will not be added after we print the
      // definitions.
      if node.variable_definitions.is_empty() {
        self.push(' ');
      }
    }

    if !node.variable_definitions.is_empty() {
      self.push('(');
      self.many(
        &node.variable_definitions,
        Printer::print_variable_definition,
        ", ",
      );
      self.push_str(") ");
    }

    self.many(
      &node.directives,
      Printer::print_directive,
      " ",
    );

    if !node.directives.is_empty() {
      self.push(' ');
    }

    self.print_selection_set(&node.selection_set);
  }

  fn print_operation_type(&mut self, operation: &ast::OperationType) {
    self.push_str(match operation {
      &ast::OperationType::Query => "query",
      &ast::OperationType::Mutation => "mutation",

      // Subscriptions are an expiremental non-spec addition.
      #[cfg(feature = "subscriptions")]
      &ast::OperationType::Subscription => "subscription",
    });
  }

  fn print_variable_definition(&mut self, node: &ast::VariableDefinition) {
    self.print_variable(&node.variable);
    self.push_str(": ");
    self.print_type_reference(&node.typ);

    if let Some(ref default_value) = node.default_value {
      self.push_str(" = ");
      self.print_value_literal(default_value);
    }
  }

  fn print_variable(&mut self, node: &ast::Variable) {
    self.push('$');
    self.print_name(&node.name);
  }

  fn print_selection_set(&mut self, node: &ast::SelectionSet) {
    self.push('{');
    self.indent();
    for selection in &node.selections {
      self.line();
      self.print_selection(&selection);
    }
    self.deindent();
    self.line();
    self.push('}');
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
      self.push_str(": ");
    }

    self.print_name(&node.name);

    if !node.arguments.is_empty() {
      self.push('(');
      self.many(
        &node.arguments,
        Printer::print_argument,
        ", ",
      );
      self.push(')');
    }

    if !node.directives.is_empty() {
      self.push(' ');
      self.many(
        &node.directives,
        Printer::print_directive,
        " ",
      );
    }

    if let Some(ref selection_set) = node.selection_set {
      self.push(' ');
      self.print_selection_set(selection_set);
    }
  }

  fn print_argument(&mut self, node: &ast::Argument) {
    self.print_name(&node.name);
    self.push_str(": ");
    self.print_value_literal(&node.value);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Fragments
  //////////////////////////////////////////////////////////////////////////////

  fn print_fragment_spread(&mut self, node: &ast::FragmentSpread) {
    self.push_str("...");
    self.print_name(&node.name);

    if !node.directives.is_empty() {
      self.push(' ');
      self.many(
        &node.directives,
        Printer::print_directive,
        " ",
      );
    }
  }

  fn print_inline_fragment(&mut self, node: &ast::InlineFragment) {
    self.push_str("...");

    if let Some(ref type_condition) = node.type_condition {
      self.push_str(" on ");
      self.print_named_type(type_condition);
    }

    if !node.directives.is_empty() {
      self.push(' ');
      self.many(
        &node.directives,
        Printer::print_directive,
        " ",
      );
    }

    self.push(' ');
    self.print_selection_set(&node.selection_set);
  }

  fn print_fragment_definition(&mut self, node: &ast::FragmentDefinition) {
    self.push_str("fragment ");
    self.print_name(&node.name);
    self.push_str(" on ");
    self.print_named_type(&node.type_condition);

    if !node.directives.is_empty() {
      self.push(' ');
      self.many(
        &node.directives,
        Printer::print_directive,
        " ",
      );
    }

    self.push(' ');
    self.print_selection_set(&node.selection_set);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Values
  //////////////////////////////////////////////////////////////////////////////

  fn print_value_literal(&mut self, node: &ast::Value) {
    match node {
      &ast::Value::Variable(ref node) => self.print_variable(node),
      &ast::Value::Int(ref node) => self.push_str(&node.value.to_string()),
      &ast::Value::Float(ref node) => self.push_str(&node.value.to_string()),
      &ast::Value::String(ref node) => self.print_string_value(node),
      &ast::Value::Boolean(ref node) => self.print_boolean_value(node),
      &ast::Value::Null(_) => self.push_str("null"),
      &ast::Value::Enum(ref node) => self.push_str(&node.value),
      &ast::Value::List(ref node) => self.print_list_value(node),
      &ast::Value::Object(ref node) => self.print_object_value(node),
    }
  }

  fn print_string_value(&mut self, node: &ast::StringValue) {
    self.push('"');
    for character in node.value.chars() {
      match character {
        '"' => {
          self.push('\\');
          self.push('"');
        },
        '\\' => {
          self.push('\\');
          self.push('\\');
        },
        '\n' => {
          self.push('\\');
          self.push('n');
        },
        character => {
          self.push(character);
        },
      }
    }
    self.push('"');
  }

  fn print_boolean_value(&mut self, node: &ast::BooleanValue) {
    if node.value {
      self.push_str("true");
    } else {
      self.push_str("false");
    }
  }

  fn print_list_value(&mut self, node: &ast::ListValue) {
    self.push('[');
    self.many(
      &node.values,
      Printer::print_value_literal,
      ", ",
    );
    self.push(']');
  }

  fn print_object_value(&mut self, node: &ast::ObjectValue) {
    self.push('{');
    self.many(
      &node.fields,
      Printer::print_object_field,
      ", ",
    );
    self.push('}');
  }

  fn print_object_field(&mut self, node: &ast::ObjectField) {
    self.print_name(&node.name);
    self.push_str(": ");
    self.print_value_literal(&node.value);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Directives
  //////////////////////////////////////////////////////////////////////////////

  fn print_directive(&mut self, node: &ast::Directive) {
    self.push('@');
    self.print_name(&node.name);

    if !node.arguments.is_empty() {
      self.push('(');
      self.many(
        &node.arguments,
        Printer::print_argument,
        ", ",
      );
      self.push(')');
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
    self.push('[');
    self.print_type_reference(&node.typ);
    self.push(']');
  }

  fn print_non_null_type(&mut self, node: &ast::NonNullType) {
    match *node.typ {
      ast::NullableType::Named(ref node) => self.print_named_type(node),
      ast::NullableType::List(ref node) => self.print_list_type(node),
    }
    self.push('!');
  }
}
