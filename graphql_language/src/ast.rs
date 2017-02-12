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
    pub struct $struct_name:ident {
      $($field_name:ident: $field_type:ty,)*
    }
  ) => (
    #[derive(Clone, PartialEq, Debug)]
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
    pub enum $enum_name:ident {
      $($variant_name:ident($variant_type:ty),)*
    }
  ) => (
    #[derive(Clone, PartialEq, Debug)]
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
  pub struct Name {
    value: String,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Document
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  pub struct Document {
    definitions: Vec<Definition>,
  }
}

// Because we have some expiremental non-spec additions to this `Definition`
// node we don’t use the `node_enum!` macro and instead manually provide the
// necessary implementations.
#[derive(Clone, PartialEq, Debug)]
pub enum Definition {
  Operation(OperationDefinition),
  Fragment(FragmentDefinition),

  // The type system AST extension is an experimental non-spec addition.
  #[cfg(feature = "type_system")]
  TypeSystem(TypeSystemDefinition),
}

impl Node for Definition {
  fn loc(&self) -> Option<&Location> {
    match *self {
      Definition::Operation(ref node) => node.loc(),
      Definition::Fragment(ref node) => node.loc(),

      // The type system AST extension is an experimental non-spec addition.
      #[cfg(feature = "type_system")]
      Definition::TypeSystem(ref node) => node.loc(),
    }
  }
}

node_struct! {
  pub struct OperationDefinition {
    operation: OperationType,
    name: Option<Name>,
    variable_definitions: Vec<VariableDefinition>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum OperationType {
  Query,
  Mutation,

  // Subscriptions are an expiremental non-spec addition.
  #[cfg(feature = "subscriptions")]
  Subscription,
}

node_struct! {
  pub struct VariableDefinition {
    variable: Variable,
    typ: Type,
    default_value: Option<Value>,
  }
}

node_struct! {
  pub struct Variable {
    name: Name,
  }
}

node_struct! {
  pub struct SelectionSet {
    selections: Vec<Selection>,
  }
}

node_enum! {
  pub enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
  }
}

node_struct! {
  pub struct Field {
    alias: Option<Name>,
    name: Name,
    arguments: Vec<Argument>,
    directives: Vec<Directive>,
    selection_set: Option<SelectionSet>,
  }
}

node_struct! {
  pub struct Argument {
    name: Name,
    value: Value,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Fragments
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  pub struct FragmentSpread {
    name: Name,
    directives: Vec<Directive>,
  }
}

node_struct! {
  pub struct InlineFragment {
    type_condition: Option<NamedType>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
  }
}

node_struct! {
  pub struct FragmentDefinition {
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
  pub enum Value {
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
  pub struct IntValue {
    value: i32,
  }
}

node_struct! {
  pub struct FloatValue {
    value: f32,
  }
}

node_struct! {
  pub struct StringValue {
    value: String,
  }
}

node_struct! {
  pub struct BooleanValue {
    value: bool,
  }
}

node_struct! {
  pub struct NullValue {}
}

node_struct! {
  pub struct EnumValue {
    value: String,
  }
}

node_struct! {
  pub struct ListValue {
    values: Vec<Value>,
  }
}

node_struct! {
  pub struct ObjectValue {
    fields: Vec<ObjectField>,
  }
}

node_struct! {
  pub struct ObjectField {
    name: Name,
    value: Value,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Directives
////////////////////////////////////////////////////////////////////////////////

node_struct! {
  pub struct Directive {
    name: Name,
    arguments: Vec<Argument>,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Type Reference
////////////////////////////////////////////////////////////////////////////////

node_enum! {
  pub enum Type {
    Named(NamedType),
    List(ListType),
    NonNull(NonNullType),
  }
}

node_enum! {
  pub enum NullableType {
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
  pub struct NamedType {
    name: Name,
  }
}

node_struct! {
  pub struct ListType {
    typ: Box<Type>,
  }
}

node_struct! {
  pub struct NonNullType {
    typ: Box<NullableType>,
  }
}

////////////////////////////////////////////////////////////////////////////////
// Type System Definition
////////////////////////////////////////////////////////////////////////////////
//
// The type system AST extension is an experimental non-spec addition.

#[cfg(feature = "type_system")]
pub use self::type_system::*;

#[cfg(feature = "type_system")]
mod type_system {
  use super::*;

  node_enum! {
    pub enum TypeSystemDefinition {
      Schema(SchemaDefinition),
      Type(TypeDefinition),
      TypeExtension(TypeExtensionDefinition),
      Directive(DirectiveDefinition),
    }
  }

  node_struct! {
    pub struct SchemaDefinition {
      directives: Vec<Directive>,
      operation_types: Vec<OperationTypeDefinition>,
    }
  }

  node_struct! {
    pub struct OperationTypeDefinition {
      operation: OperationType,
      typ: NamedType,
    }
  }

  node_enum! {
    pub enum TypeDefinition {
      Scalar(ScalarTypeDefinition),
      Object(ObjectTypeDefinition),
      Interface(InterfaceTypeDefinition),
      Union(UnionTypeDefinition),
      Enum(EnumTypeDefinition),
      InputObject(InputObjectTypeDefinition),
    }
  }

  node_struct! {
    pub struct ScalarTypeDefinition {
      name: Name,
      directives: Vec<Directive>,
    }
  }

  node_struct! {
    pub struct ObjectTypeDefinition {
      name: Name,
      interfaces: Vec<NamedType>,
      directives: Vec<Directive>,
      fields: Vec<FieldDefinition>,
    }
  }

  node_struct! {
    pub struct FieldDefinition {
      name: Name,
      arguments: Vec<InputValueDefinition>,
      typ: Type,
      directives: Vec<Directive>,
    }
  }

  node_struct! {
    pub struct InputValueDefinition {
      name: Name,
      typ: Type,
      default_value: Option<Value>,
      directives: Vec<Directive>,
    }
  }

  node_struct! {
    pub struct InterfaceTypeDefinition {
      name: Name,
      directives: Vec<Directive>,
      fields: Vec<FieldDefinition>,
    }
  }

  node_struct! {
    pub struct UnionTypeDefinition {
      name: Name,
      directives: Vec<Directive>,
      types: Vec<NamedType>,
    }
  }

  node_struct! {
    pub struct EnumTypeDefinition {
      name: Name,
      directives: Vec<Directive>,
      values: Vec<EnumValueDefinition>,
    }
  }

  node_struct! {
    pub struct EnumValueDefinition {
      name: Name,
      directives: Vec<Directive>,
    }
  }

  node_struct! {
    pub struct InputObjectTypeDefinition {
      name: Name,
      directives: Vec<Directive>,
      fields: Vec<InputValueDefinition>,
    }
  }

  node_struct! {
    pub struct TypeExtensionDefinition {
      definition: ObjectTypeDefinition,
    }
  }

  node_struct! {
    pub struct DirectiveDefinition {
      name: Name,
      arguments: Vec<InputValueDefinition>,
      locations: Vec<Name>,
    }
  }
}
