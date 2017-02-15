// TODO: This is temporary. Rewrite when we better understand the use case.

use std::collections::HashMap;

pub struct Schema;

pub enum Type<'a> {
  Scalar(ScalarType),
  Object(ObjectType<'a>),
  Interface(InterfaceType<'a>),
  Union(UnionType<'a>),
  Enum(EnumType),
  InputObject(InputObjectType<'a>),
  List(Box<Type<'a>>),
  NonNull(Box<Type<'a>>),
}

pub enum InputType<'a> {
  Scalar(ScalarType),
  Enum(EnumType),
  InputObject(InputObjectType<'a>),
  List(Box<InputType<'a>>),
  NonNull(Box<InputType<'a>>),
}

pub enum OutputType<'a> {
  Scalar(ScalarType),
  Object(ObjectType<'a>),
  Interface(InterfaceType<'a>),
  Union(UnionType<'a>),
  Enum(EnumType),
  List(Box<OutputType<'a>>),
  NonNull(Box<OutputType<'a>>),
}

pub struct ScalarType {
  name: String,
  description: Option<String>,
}

impl ScalarType {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }
}

pub struct ObjectType<'a> {
  name: String,
  description: Option<String>,
  interfaces: Vec<&'a InterfaceType<'a>>,
  // TODO: fields order
  fields: HashMap<String, Field<'a>>,
}

impl<'a> ObjectType<'a> {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn interfaces(&self) -> &Vec<&'a InterfaceType> {
    &self.interfaces
  }

  pub fn field(&self, name: &str) -> Option<&Field<'a>> {
    self.fields.get(name)
  }
}

pub struct Field<'a> {
  description: Option<String>,
  args: HashMap<String, InputValue<'a>>,
  type_: &'a OutputType<'a>,
}

impl<'a> Field<'a> {
  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn type_(&self) -> &'a OutputType<'a> {
    self.type_
  }

  pub fn arg(&self, name: &str) -> Option<&InputValue<'a>> {
    self.args.get(name)
  }
}

pub struct InputValue<'a> {
  description: Option<String>,
  type_: &'a InputType<'a>,
  // TODO: default_value
}

impl<'a> InputValue<'a> {
  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn type_(&self) -> &'a InputType<'a> {
    self.type_
  }
}

pub struct InterfaceType<'a> {
  name: String,
  description: Option<String>,
  // TODO: fields order
  fields: HashMap<String, Field<'a>>,
}

impl<'a> InterfaceType<'a> {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn field(&self, name: &str) -> Option<&Field<'a>> {
    self.fields.get(name)
  }
}

pub struct UnionType<'a> {
  name: String,
  description: Option<String>,
  types: Vec<&'a ObjectType<'a>>,
}

impl<'a> UnionType<'a> {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn types(&self) -> &Vec<&'a ObjectType> {
    &self.types
  }
}

pub struct EnumType {
  name: String,
  description: Option<String>,
  values: HashMap<String, EnumValue>,
}

impl EnumType {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }
}

pub struct EnumValue {
  description: Option<String>,
}

impl EnumValue {
  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }
}

pub struct InputObjectType<'a> {
  name: String,
  description: Option<String>,
  fields: HashMap<String, InputValue<'a>>,
}

impl<'a> InputObjectType<'a> {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn field(&self, name: &str) -> Option<&InputValue<'a>> {
    self.fields.get(name)
  }
}
