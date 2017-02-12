//! The data types that represent an AST of a GraphQL document.
//!
//! The root AST type is `Document`.

use super::Location;

/// A node in the GraphQL AST represents any data structure or enumeration of
/// data structure types.
pub trait Node {
  /// The location at which thie AST node is located in its source document.
  fn loc(&self) -> Option<&Location>;
}

/// Creates a node struct with all of the repetetive code that is required.
///
/// The goal is to make this macro look as close to an actual struct definition
/// as possible.
macro_rules! node_struct {
  (
    struct $struct_name:ident {
      $($field_name:ident: $field_type:ty,)*
    }
  ) => (
    #[derive(PartialEq, Debug)]
    pub struct $struct_name {
      pub loc: Option<Location>,
      $(
        pub $field_name: $field_type,
      )*
    }

    impl Node for $struct_name {
      fn loc(&self) -> Option<&Location> {
        self.loc.as_ref()
      }
    }
  )
}

/// Creates a node enum with all of the repetetive code that is required.
///
/// The goal is to make this macro look as close to an actual enum definition
/// as possible.
macro_rules! node_enum {
  (
    enum $enum_name:ident {
      $($variant_name:ident($variant_type:ty),)*
    }
  ) => (
    #[derive(PartialEq, Debug)]
    pub enum $enum_name {
      $(
        $variant_name($variant_type),
      )*
    }

    impl Node for $enum_name {
      fn loc(&self) -> Option<&Location> {
        match *self {
          $(
            $enum_name::$variant_name(ref node) => node.loc(),
          )*
        }
      }
    }
  )
}

////////////////////////////////////////////////////////////////////////////////
// Name
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  struct Name {
    value: String,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Document
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  struct Document {
    definitions: Vec<Definition>,
  }
}

node_enum! {
  enum Definition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    TypeSystem(TypeSystemDefinition),
  }
}

node_struct! {
  struct OperationDefinition {
    operation: OperationType,
    name: Option<Name>,
    variable_definitions: Vec<VariableDefinition>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
  }
}

#[derive(PartialEq, Debug)]
pub enum OperationType {
  Query,
  Mutation,

  // Only allow a subscriptions operation type if the feature were enabled.
  #[cfg(feature = "subscriptions")]
  Subscription,
}

node_struct! {
  struct VariableDefinition {
    variable: Variable,
    typ: Type,
    default_value: Option<Value>,
  }
}

node_struct! {
  struct Variable {
    name: Name,
  }
}

node_struct! {
  struct SelectionSet {
    selections: Vec<Selection>,
  }
}

node_enum! {
  enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
  }
}

node_struct! {
  struct Field {
    alias: Option<Name>,
    name: Name,
    arguments: Vec<Argument>,
    directives: Vec<Directive>,
    selection_set: Option<SelectionSet>,
  }
}

node_struct! {
  struct Argument {
    name: Name,
    value: Value,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Fragments
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  struct FragmentSpread {
    name: Name,
    directives: Vec<Directive>,
  }
}

node_struct! {
  struct InlineFragment {
    type_condition: Option<NamedType>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
  }
}

node_struct! {
  struct FragmentDefinition {
    name: Name,
    type_condition: NamedType,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Values
////////////////////////////////////////////////////////////////////////////////

node_enum! {
  enum Value {
    Variable(Variable),
    Int(IntValue),
    Float(FloatValue),
    String(StringValue),
    Boolean(BooleanValue),
    Null(NullValue),
    Enum(EnumValue),
    List(ListValue),
    Object(ObjectValue),
  }
}

node_struct! {
  struct IntValue {
    value: i32,
  }
}

node_struct! {
  struct FloatValue {
    value: f32,
  }
}

node_struct! {
  struct StringValue {
    value: String,
  }
}

node_struct! {
  struct BooleanValue {
    value: bool,
  }
}

node_struct! {
  struct NullValue {}
}

node_struct! {
  struct EnumValue {
    value: String,
  }
}

node_struct! {
  struct ListValue {
    values: Vec<Value>,
  }
}

node_struct! {
  struct ObjectValue {
    fields: Vec<ObjectField>,
  }
}

node_struct! {
  struct ObjectField {
    name: Name,
    value: Value,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Directives
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  struct Directive {
    name: Name,
    arguments: Vec<Argument>,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Type Reference
////////////////////////////////////////////////////////////////////////////////

node_enum! {
  enum Type {
    Named(NamedType),
    List(ListType),
    NonNull(NonNullType),
  }
}

node_enum! {
  enum NullableType {
    Named(NamedType),
    List(ListType),
  }
}

impl From<NullableType> for Type {
  fn from(typ: NullableType) -> Type {
    match typ {
      NullableType::Named(named) => Type::Named(named),
      NullableType::List(list) => Type::List(list),
    }
  }
}

node_struct! {
  struct NamedType {
    name: Name,
  }
}

node_struct! {
  struct ListType {
    typ: Box<Type>,
  }
}

node_struct! {
  struct NonNullType {
    typ: Box<NullableType>,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Type System Definition
////////////////////////////////////////////////////////////////////////////////

node_enum! {
  enum TypeSystemDefinition {
    Schema(SchemaDefinition),
    Type(TypeDefinition),
    TypeExtension(TypeExtensionDefinition),
    Directive(DirectiveDefinition),
  }
}

node_struct! {
  struct SchemaDefinition {
    directives: Vec<Directive>,
    operation_types: Vec<OperationTypeDefinition>,
  }
}

node_struct! {
  struct OperationTypeDefinition {
    operation: OperationType,
    typ: NamedType,
  }
}

node_enum! {
  enum TypeDefinition {
    Scalar(ScalarTypeDefinition),
    Object(ObjectTypeDefinition),
    Interface(InterfaceTypeDefinition),
    Union(UnionTypeDefinition),
    Enum(EnumTypeDefinition),
    InputObject(InputObjectTypeDefinition),
  }
}

node_struct! {
  struct ScalarTypeDefinition {
    name: Name,
    directives: Vec<Directive>,
  }
}

node_struct! {
  struct ObjectTypeDefinition {
    name: Name,
    interfaces: Vec<NamedType>,
    directives: Vec<Directive>,
    fields: Vec<FieldDefinition>,
  }
}

node_struct! {
  struct FieldDefinition {
    name: Name,
    arguments: Vec<InputValueDefinition>,
    typ: Type,
    directives: Vec<Directive>,
  }
}

node_struct! {
  struct InputValueDefinition {
    name: Name,
    typ: Type,
    default_value: Value,
    directives: Vec<Directive>,
  }
}

node_struct! {
  struct InterfaceTypeDefinition {
    name: Name,
    directives: Vec<Directive>,
    fields: Vec<FieldDefinition>,
  }
}

node_struct! {
  struct UnionTypeDefinition {
    name: Name,
    directives: Vec<Directive>,
    types: Vec<NamedType>,
  }
}

node_struct! {
  struct EnumTypeDefinition {
    name: Name,
    directives: Vec<Directive>,
    values: Vec<EnumValueDefinition>,
  }
}

node_struct! {
  struct EnumValueDefinition {
    name: Name,
    directives: Vec<Directive>,
  }
}

node_struct! {
  struct InputObjectTypeDefinition {
    name: Name,
    directives: Vec<Directive>,
    fields: Vec<InputValueDefinition>,
  }
}

node_struct! {
  struct TypeExtensionDefinition {
    definition: ObjectTypeDefinition,
  }
}

node_struct! {
  struct DirectiveDefinition {
    name: Name,
    arguments: Vec<InputValueDefinition>,
    locations: Vec<Name>,
  }
}
