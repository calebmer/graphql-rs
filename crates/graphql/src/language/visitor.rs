use super::ast::*;

/// Runs a visitor through a GraphQL AST returning the new AST created by
/// mutations from the visitor.
pub fn visit<V: Visitor>(visitor: &mut V, document: Document) -> Document {
  visit_document(visitor, document)
}

/// A utility for generating two functions to enter and leave a node on the
/// `Visitor` trait.
macro_rules! fn_visits {
  ($type_:ty, $enter_fn:ident, $leave_fn:ident) => (
    fn $enter_fn(&mut self, node: $type_) -> $type_ { node }
    fn $leave_fn(&mut self, node: $type_) -> $type_ { node }
  )
}

/// A visitor can be used to traverse and mutate a GraphQL AST.
///
/// All of the functions have a noop default implementation. Only implement the
/// functions where you want custom behavior to occur when visiting.
pub trait Visitor {
  fn_visits!(Name, enter_name, leave_name);
  fn_visits!(Document, enter_document, leave_document);
  fn_visits!(Definition, enter_definition, leave_definition);
  fn_visits!(OperationDefinition, enter_operation_definition, leave_operation_definition);
  fn_visits!(VariableDefinition, enter_variable_definition, leave_variable_definition);
  fn_visits!(Variable, enter_variable, leave_variable);
  fn_visits!(SelectionSet, enter_selection_set, leave_selection_set);
  fn_visits!(Selection, enter_selection, leave_selection);
  fn_visits!(Field, enter_field, leave_field);
  fn_visits!(Argument, enter_argument, leave_argument);
  fn_visits!(FragmentSpread, enter_fragment_spread, leave_fragment_spread);
  fn_visits!(InlineFragment, enter_inline_fragment, leave_inline_fragment);
  fn_visits!(FragmentDefinition, enter_fragment_definition, leave_fragment_definition);
  fn_visits!(Value, enter_value, leave_value);
  fn_visits!(IntValue, enter_int_value, leave_int_value);
  fn_visits!(FloatValue, enter_float_value, leave_float_value);
  fn_visits!(StringValue, enter_string_value, leave_string_value);
  fn_visits!(BooleanValue, enter_boolean_value, leave_boolean_value);
  fn_visits!(NullValue, enter_null_value, leave_null_value);
  fn_visits!(EnumValue, enter_enum_value, leave_enum_value);
  fn_visits!(ListValue, enter_list_value, leave_list_value);
  fn_visits!(ObjectValue, enter_object_value, leave_object_value);
  fn_visits!(ObjectField, enter_object_field, leave_object_field);
  fn_visits!(Directive, enter_directive, leave_directive);
  fn_visits!(Type, enter_type, leave_type);
  fn_visits!(NullableType, enter_nullable_type, leave_nullable_type);
  fn_visits!(NamedType, enter_named_type, leave_named_type);
  fn_visits!(ListType, enter_list_type, leave_list_type);
  fn_visits!(NonNullType, enter_non_null_type, leave_non_null_type);
}

/// A visitor that will take any number of other visitors and execute their
/// enter/leave functions in parallel. This is *much* more efficient then
/// visiting an AST tree with each visitor individually.
pub struct ParallelVisitor<'a> {
  visitors: Vec<&'a mut Visitor>,
}

impl<'a> ParallelVisitor<'a> {
  /// Creates a new parallel visitor using some mutable references to visitors.
  pub fn new(visitors: Vec<&'a mut Visitor>) -> Self {
    ParallelVisitor {
      visitors: visitors,
    }
  }
}

/// A utility for generating two functions to enter and leave a node with
/// multiple visitors in parallel on `ParallelVisitor`.
macro_rules! fn_visits_parallel {
  ($type_:ty, $enter_fn:ident, $leave_fn:ident) => (
    fn $enter_fn(&mut self, _node: $type_) -> $type_ {
      let mut node = _node;
      for visitor in self.visitors.iter_mut() {
        node = visitor.$enter_fn(node);
      }
      node
    }

    fn $leave_fn(&mut self, _node: $type_) -> $type_ {
      let mut node = _node;
      for visitor in self.visitors.iter_mut().rev() {
        node = visitor.$leave_fn(node);
      }
      node
    }
  )
}

impl<'a> Visitor for ParallelVisitor<'a> {
  fn_visits_parallel!(Name, enter_name, leave_name);
  fn_visits_parallel!(Document, enter_document, leave_document);
  fn_visits_parallel!(Definition, enter_definition, leave_definition);
  fn_visits_parallel!(OperationDefinition, enter_operation_definition, leave_operation_definition);
  fn_visits_parallel!(VariableDefinition, enter_variable_definition, leave_variable_definition);
  fn_visits_parallel!(Variable, enter_variable, leave_variable);
  fn_visits_parallel!(SelectionSet, enter_selection_set, leave_selection_set);
  fn_visits_parallel!(Selection, enter_selection, leave_selection);
  fn_visits_parallel!(Field, enter_field, leave_field);
  fn_visits_parallel!(Argument, enter_argument, leave_argument);
  fn_visits_parallel!(FragmentSpread, enter_fragment_spread, leave_fragment_spread);
  fn_visits_parallel!(InlineFragment, enter_inline_fragment, leave_inline_fragment);
  fn_visits_parallel!(FragmentDefinition, enter_fragment_definition, leave_fragment_definition);
  fn_visits_parallel!(Value, enter_value, leave_value);
  fn_visits_parallel!(IntValue, enter_int_value, leave_int_value);
  fn_visits_parallel!(FloatValue, enter_float_value, leave_float_value);
  fn_visits_parallel!(StringValue, enter_string_value, leave_string_value);
  fn_visits_parallel!(BooleanValue, enter_boolean_value, leave_boolean_value);
  fn_visits_parallel!(NullValue, enter_null_value, leave_null_value);
  fn_visits_parallel!(EnumValue, enter_enum_value, leave_enum_value);
  fn_visits_parallel!(ListValue, enter_list_value, leave_list_value);
  fn_visits_parallel!(ObjectValue, enter_object_value, leave_object_value);
  fn_visits_parallel!(ObjectField, enter_object_field, leave_object_field);
  fn_visits_parallel!(Directive, enter_directive, leave_directive);
  fn_visits_parallel!(Type, enter_type, leave_type);
  fn_visits_parallel!(NullableType, enter_nullable_type, leave_nullable_type);
  fn_visits_parallel!(NamedType, enter_named_type, leave_named_type);
  fn_visits_parallel!(ListType, enter_list_type, leave_list_type);
  fn_visits_parallel!(NonNullType, enter_non_null_type, leave_non_null_type);
}

// Below this comment is the actual visiting implementation code.

////////////////////////////////////////////////////////////////////////////////
// Utilities
////////////////////////////////////////////////////////////////////////////////

/// A utility for generating code to visit many nodes. This code may get verbose
/// and so a macro is helpful.
macro_rules! visit_many {
  ($fn_:ident, $visitor:expr, $many:expr) => ({
    $many = $many.into_iter().map(|value| $fn_($visitor, value)).collect();
  })
}

////////////////////////////////////////////////////////////////////////////////
// Name
////////////////////////////////////////////////////////////////////////////////

fn visit_name<V: Visitor>(v: &mut V, name: Name) -> Name {
  let mut node = name;
  node = v.enter_name(node);
  v.leave_name(node)
}

////////////////////////////////////////////////////////////////////////////////
// Document
////////////////////////////////////////////////////////////////////////////////

fn visit_document<V: Visitor>(v: &mut V, _node: Document) -> Document {
  let mut node = _node;
  node = v.enter_document(node);
  visit_many!(visit_definition, v, node.definitions);
  v.leave_document(node)
}

fn visit_definition<V: Visitor>(v: &mut V, _node: Definition) -> Definition {
  let mut node = _node;
  node = v.enter_definition(node);
  node = match node {
    Definition::Operation(operation) => Definition::Operation(visit_operation_definition(v, operation)),
    Definition::Fragment(fragment) => Definition::Fragment(visit_fragment_definition(v, fragment)),
  };
  v.leave_definition(node)
}

fn visit_operation_definition<V: Visitor>(v: &mut V, _node: OperationDefinition) -> OperationDefinition {
  let mut node = _node;
  node = v.enter_operation_definition(node);
  node.name = node.name.map(|name| visit_name(v, name));
  visit_many!(visit_variable_definition, v, node.variable_definitions);
  visit_many!(visit_directive, v, node.directives);
  node.selection_set = visit_selection_set(v, node.selection_set);
  v.leave_operation_definition(node)
}

fn visit_variable_definition<V: Visitor>(v: &mut V, _node: VariableDefinition) -> VariableDefinition {
  let mut node = _node;
  node = v.enter_variable_definition(node);
  node.variable = visit_variable(v, node.variable);
  node.type_ = visit_type(v, node.type_);
  node.default_value = node.default_value.map(|default_value| visit_value(v, default_value));
  v.leave_variable_definition(node)
}

fn visit_variable<V: Visitor>(v: &mut V, _node: Variable) -> Variable {
  let mut node = _node;
  node = v.enter_variable(node);
  node.name = visit_name(v, node.name);
  v.leave_variable(node)
}

fn visit_selection_set<V: Visitor>(v: &mut V, _node: SelectionSet) -> SelectionSet {
  let mut node = _node;
  node = v.enter_selection_set(node);
  visit_many!(visit_selection, v, node.selections);
  v.leave_selection_set(node)
}

fn visit_selection<V: Visitor>(v: &mut V, _node: Selection) -> Selection {
  let mut node = _node;
  node = v.enter_selection(node);
  node = match node {
    Selection::Field(field) => Selection::Field(visit_field(v, field)),
    Selection::FragmentSpread(fragment_spread) => Selection::FragmentSpread(visit_fragment_spread(v, fragment_spread)),
    Selection::InlineFragment(inline_fragment) => Selection::InlineFragment(visit_inline_fragment(v, inline_fragment)),
  };
  v.leave_selection(node)
}

fn visit_field<V: Visitor>(v: &mut V, _node: Field) -> Field {
  let mut node = _node;
  node = v.enter_field(node);
  node.alias = node.alias.map(|alias| visit_name(v, alias));
  node.name = visit_name(v, node.name);
  visit_many!(visit_argument, v, node.arguments);
  visit_many!(visit_directive, v, node.directives);
  node.selection_set = node.selection_set.map(|selection_set| visit_selection_set(v, selection_set));
  v.leave_field(node)
}

fn visit_argument<V: Visitor>(v: &mut V, _node: Argument) -> Argument {
  let mut node = _node;
  node = v.enter_argument(node);
  node.name = visit_name(v, node.name);
  node.value = visit_value(v, node.value);
  v.leave_argument(node)
}

////////////////////////////////////////////////////////////////////////////////
// Fragments
////////////////////////////////////////////////////////////////////////////////

fn visit_fragment_spread<V: Visitor>(v: &mut V, _node: FragmentSpread) -> FragmentSpread {
  let mut node = _node;
  node = v.enter_fragment_spread(node);
  node.name = visit_name(v, node.name);
  visit_many!(visit_directive, v, node.directives);
  v.leave_fragment_spread(node)
}

fn visit_inline_fragment<V: Visitor>(v: &mut V, _node: InlineFragment) -> InlineFragment {
  let mut node = _node;
  node = v.enter_inline_fragment(node);
  node.type_condition = node.type_condition.map(|type_condition| visit_named_type(v, type_condition));
  visit_many!(visit_directive, v, node.directives);
  node.selection_set = visit_selection_set(v, node.selection_set);
  v.leave_inline_fragment(node)
}

fn visit_fragment_definition<V: Visitor>(v: &mut V, _node: FragmentDefinition) -> FragmentDefinition {
  let mut node = _node;
  node = v.enter_fragment_definition(node);
  node.name = visit_name(v, node.name);
  node.type_condition = visit_named_type(v, node.type_condition);
  visit_many!(visit_directive, v, node.directives);
  node.selection_set = visit_selection_set(v, node.selection_set);
  v.leave_fragment_definition(node)
}

////////////////////////////////////////////////////////////////////////////////
// Values
////////////////////////////////////////////////////////////////////////////////

fn visit_value<V: Visitor>(v: &mut V, _node: Value) -> Value {
  let mut node = _node;
  node = v.enter_value(node);
  node = match node {
    Value::Variable(node) => Value::Variable(visit_variable(v, node)),
    Value::Int(node) => Value::Int(visit_int_value(v, node)),
    Value::Float(node) => Value::Float(visit_float_value(v, node)),
    Value::String(node) => Value::String(visit_string_value(v, node)),
    Value::Boolean(node) => Value::Boolean(visit_boolean_value(v, node)),
    Value::Null(node) => Value::Null(visit_null_value(v, node)),
    Value::Enum(node) => Value::Enum(visit_enum_value(v, node)),
    Value::List(node) => Value::List(visit_list_value(v, node)),
    Value::Object(node) => Value::Object(visit_object_value(v, node)),
  };
  v.leave_value(node)
}

fn visit_int_value<V: Visitor>(v: &mut V, _node: IntValue) -> IntValue {
  let mut node = _node;
  node = v.enter_int_value(node);
  v.leave_int_value(node)
}

fn visit_float_value<V: Visitor>(v: &mut V, _node: FloatValue) -> FloatValue {
  let mut node = _node;
  node = v.enter_float_value(node);
  v.leave_float_value(node)
}

fn visit_string_value<V: Visitor>(v: &mut V, _node: StringValue) -> StringValue {
  let mut node = _node;
  node = v.enter_string_value(node);
  v.leave_string_value(node)
}

fn visit_boolean_value<V: Visitor>(v: &mut V, _node: BooleanValue) -> BooleanValue {
  let mut node = _node;
  node = v.enter_boolean_value(node);
  v.leave_boolean_value(node)
}

fn visit_null_value<V: Visitor>(v: &mut V, _node: NullValue) -> NullValue {
  let mut node = _node;
  node = v.enter_null_value(node);
  v.leave_null_value(node)
}

fn visit_enum_value<V: Visitor>(v: &mut V, _node: EnumValue) -> EnumValue {
  let mut node = _node;
  node = v.enter_enum_value(node);
  v.leave_enum_value(node)
}

fn visit_list_value<V: Visitor>(v: &mut V, _node: ListValue) -> ListValue {
  let mut node = _node;
  node = v.enter_list_value(node);
  visit_many!(visit_value, v, node.values);
  v.leave_list_value(node)
}

fn visit_object_value<V: Visitor>(v: &mut V, _node: ObjectValue) -> ObjectValue {
  let mut node = _node;
  node = v.enter_object_value(node);
  visit_many!(visit_object_field, v, node.fields);
  v.leave_object_value(node)
}

fn visit_object_field<V: Visitor>(v: &mut V, _node: ObjectField) -> ObjectField {
  let mut node = _node;
  node = v.enter_object_field(node);
  node.name = visit_name(v, node.name);
  node.value = visit_value(v, node.value);
  v.leave_object_field(node)
}

////////////////////////////////////////////////////////////////////////////////
// Directives
////////////////////////////////////////////////////////////////////////////////

fn visit_directive<V: Visitor>(v: &mut V, _node: Directive) -> Directive {
  let mut node = _node;
  node = v.enter_directive(node);
  node.name = visit_name(v, node.name);
  visit_many!(visit_argument, v, node.arguments);
  v.leave_directive(node)
}

////////////////////////////////////////////////////////////////////////////////
// Type Reference
////////////////////////////////////////////////////////////////////////////////

fn visit_type<V: Visitor>(v: &mut V, _node: Type) -> Type {
  let mut node = _node;
  node = v.enter_type(node);
  node = match node {
    Type::Named(node) => Type::Named(visit_named_type(v, node)),
    Type::List(node) => Type::List(visit_list_type(v, node)),
    Type::NonNull(node) => Type::NonNull(visit_non_null_type(v, node)),
  };
  v.leave_type(node)
}

fn visit_nullable_type<V: Visitor>(v: &mut V, _node: NullableType) -> NullableType {
  let mut node = _node;
  node = v.enter_nullable_type(node);
  node = match node {
    NullableType::Named(node) => NullableType::Named(visit_named_type(v, node)),
    NullableType::List(node) => NullableType::List(visit_list_type(v, node)),
  };
  v.leave_nullable_type(node)
}

fn visit_named_type<V: Visitor>(v: &mut V, _node: NamedType) -> NamedType {
  let mut node = _node;
  node = v.enter_named_type(node);
  node.name = visit_name(v, node.name);
  v.leave_named_type(node)
}

fn visit_list_type<V: Visitor>(v: &mut V, _node: ListType) -> ListType {
  let mut node = _node;
  node = v.enter_list_type(node);
  node.type_ = Box::new(visit_type(v, *node.type_));
  v.leave_list_type(node)
}

fn visit_non_null_type<V: Visitor>(v: &mut V, _node: NonNullType) -> NonNullType {
  let mut node = _node;
  node = v.enter_non_null_type(node);
  node.type_ = Box::new(visit_nullable_type(v, *node.type_));
  v.leave_non_null_type(node)
}
