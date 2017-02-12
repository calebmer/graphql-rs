#![cfg(feature = "type_system")]

extern crate graphql_language;

use std::io::prelude::*;
use std::fs::File;
use graphql_language::parse_without_location;
use graphql_language::ast::*;

#[test]
fn parse_kitchen_sink_schema() {
  let mut file = File::open("tests/fixtures/kitchen-sink-schema.graphql").unwrap();
  let mut string = String::new();
  file.read_to_string(&mut string).unwrap();
  let ast = parse_without_location(string.chars());
  assert_eq!(ast, Ok(Document {
    loc: None,
    definitions: vec![
      Definition::TypeSystem(TypeSystemDefinition::Schema(SchemaDefinition {
        loc: None,
        directives: vec![],
        operation_types: vec![
          OperationTypeDefinition {
            loc: None,
            operation: OperationType::Query,
            typ: NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("QueryType"),
              },
            },
          },
          OperationTypeDefinition {
            loc: None,
            operation: OperationType::Mutation,
            typ: NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("MutationType"),
              },
            },
          },
        ],
      })),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Object(ObjectTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("Foo"),
        },
        interfaces: vec![
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("Bar"),
            },
          },
        ],
        directives: vec![],
        fields: vec![
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("one"),
            },
            arguments: vec![],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("two"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::NonNull(NonNullType {
                  loc: None,
                  typ: Box::new(NullableType::Named(NamedType {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("InputType"),
                    },
                  })),
                }),
                default_value: None,
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("three"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("InputType"),
                  },
                }),
                default_value: None,
                directives: vec![],
              },
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("other"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("String"),
                  },
                }),
                default_value: None,
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Int"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("four"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("String"),
                  },
                }),
                default_value: Some(Value::String(StringValue {
                  loc: None,
                  value: String::from("string"),
                })),
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("String"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("five"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::List(ListType {
                  loc: None,
                  typ: Box::new(Type::Named(NamedType {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("String"),
                    },
                  })),
                }),
                default_value: Some(Value::List(ListValue {
                  loc: None,
                  values: vec![
                    Value::String(StringValue {
                      loc: None,
                      value: String::from("string"),
                    }),
                    Value::String(StringValue {
                      loc: None,
                      value: String::from("string"),
                    }),
                  ],
                })),
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("String"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("six"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("InputType"),
                  },
                }),
                default_value: Some(Value::Object(ObjectValue {
                  loc: None,
                  fields: vec![
                    ObjectField {
                      loc: None,
                      name: Name {
                        loc: None,
                        value: String::from("key"),
                      },
                      value: Value::String(StringValue {
                        loc: None,
                        value: String::from("value"),
                      }),
                    },
                  ],
                })),
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("seven"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("Int"),
                  },
                }),
                default_value: Some(Value::Null(NullValue {
                  loc: None,
                })),
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Object(ObjectTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedObject"),
        },
        interfaces: vec![],
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onObject"),
            },
            arguments: vec![
              Argument {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("arg"),
                },
                value: Value::String(StringValue {
                  loc: None,
                  value: String::from("value"),
                }),
              },
            ],
          },
        ],
        fields: vec![
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("annotatedField"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("arg"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("Type"),
                  },
                }),
                default_value: Some(Value::String(StringValue {
                  loc: None,
                  value: String::from("default"),
                })),
                directives: vec![
                  Directive {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("onArg"),
                    },
                    arguments: vec![],
                  },
                ],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![
              Directive {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("onField"),
                },
                arguments: vec![],
              },
            ],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Interface(InterfaceTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("Bar"),
        },
        directives: vec![],
        fields: vec![
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("one"),
            },
            arguments: vec![],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![],
          },
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("four"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("argument"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("String"),
                  },
                }),
                default_value: Some(Value::String(StringValue {
                  loc: None,
                  value: String::from("string"),
                })),
                directives: vec![],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("String"),
              },
            }),
            directives: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Interface(InterfaceTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedInterface"),
        },
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onInterface"),
            },
            arguments: vec![],
          },
        ],
        fields: vec![
          FieldDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("annotatedField"),
            },
            arguments: vec![
              InputValueDefinition {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("arg"),
                },
                typ: Type::Named(NamedType {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("Type"),
                  },
                }),
                default_value: None,
                directives: vec![
                  Directive {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("onArg"),
                    },
                    arguments: vec![],
                  },
                ],
              },
            ],
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            directives: vec![
              Directive {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("onField"),
                },
                arguments: vec![],
              },
            ],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Union(UnionTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("Feed"),
        },
        directives: vec![],
        types: vec![
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("Story"),
            },
          },
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("Article"),
            },
          },
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("Advert"),
            },
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Union(UnionTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedUnion"),
        },
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onUnion"),
            },
            arguments: vec![],
          },
        ],
        types: vec![
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("A"),
            },
          },
          NamedType {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("B"),
            },
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Scalar(ScalarTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("CustomScalar"),
        },
        directives: vec![],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Scalar(ScalarTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedScalar"),
        },
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onScalar"),
            },
            arguments: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Enum(EnumTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("Site"),
        },
        directives: vec![],
        values: vec![
          EnumValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("DESKTOP"),
            },
            directives: vec![],
          },
          EnumValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("MOBILE"),
            },
            directives: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Enum(EnumTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedEnum"),
        },
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onEnum"),
            },
            arguments: vec![],
          },
        ],
        values: vec![
          EnumValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("ANNOTATED_VALUE"),
            },
            directives: vec![
              Directive {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("onEnumValue"),
                },
                arguments: vec![],
              },
            ],
          },
          EnumValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("OTHER_VALUE"),
            },
            directives: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::InputObject(InputObjectTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("InputType"),
        },
        directives: vec![],
        fields: vec![
          InputValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("key"),
            },
            typ: Type::NonNull(NonNullType {
              loc: None,
              typ: Box::new(NullableType::Named(NamedType {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("String"),
                },
              })),
            }),
            default_value: None,
            directives: vec![],
          },
          InputValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("answer"),
            },
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Int"),
              },
            }),
            default_value: Some(Value::Int(IntValue {
              loc: None,
              value: 42,
            })),
            directives: vec![],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::InputObject(InputObjectTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("AnnotatedInput"),
        },
        directives: vec![
          Directive {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("onInputObjectType"),
            },
            arguments: vec![],
          },
        ],
        fields: vec![
          InputValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("annotatedField"),
            },
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Type"),
              },
            }),
            default_value: None,
            directives: vec![
              Directive {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("onField"),
                },
                arguments: vec![],
              },
            ],
          },
        ],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::TypeExtension(TypeExtensionDefinition {
        loc: None,
        definition: ObjectTypeDefinition {
          loc: None,
          name: Name {
            loc: None,
            value: String::from("Foo"),
          },
          interfaces: vec![],
          directives: vec![],
          fields: vec![
            FieldDefinition {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("seven"),
              },
              arguments: vec![
                InputValueDefinition {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("argument"),
                  },
                  typ: Type::List(ListType {
                    loc: None,
                    typ: Box::new(Type::Named(NamedType {
                      loc: None,
                      name: Name {
                        loc: None,
                        value: String::from("String"),
                      },
                    })),
                  }),
                  default_value: None,
                  directives: vec![],
                },
              ],
              typ: Type::Named(NamedType {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("Type"),
                },
              }),
              directives: vec![],
            },
          ],
        },
      })),
      Definition::TypeSystem(TypeSystemDefinition::TypeExtension(TypeExtensionDefinition {
        loc: None,
        definition: ObjectTypeDefinition {
          loc: None,
          name: Name {
            loc: None,
            value: String::from("Foo"),
          },
          interfaces: vec![],
          directives: vec![
            Directive {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("onType"),
              },
              arguments: vec![],
            },
          ],
          fields: vec![],
        },
      })),
      Definition::TypeSystem(TypeSystemDefinition::Type(TypeDefinition::Object(ObjectTypeDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("NoFields"),
        },
        interfaces: vec![],
        directives: vec![],
        fields: vec![],
      }))),
      Definition::TypeSystem(TypeSystemDefinition::Directive(DirectiveDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("skip"),
        },
        arguments: vec![
          InputValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("if"),
            },
            typ: Type::NonNull(NonNullType {
              loc: None,
              typ: Box::new(NullableType::Named(NamedType {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("Boolean"),
                },
              })),
            }),
            default_value: None,
            directives: vec![],
          },
        ],
        locations: vec![
          Name {
            loc: None,
            value: String::from("FIELD"),
          },
          Name {
            loc: None,
            value: String::from("FRAGMENT_SPREAD"),
          },
          Name {
            loc: None,
            value: String::from("INLINE_FRAGMENT"),
          },
        ],
      })),
      Definition::TypeSystem(TypeSystemDefinition::Directive(DirectiveDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("include"),
        },
        arguments: vec![
          InputValueDefinition {
            loc: None,
            name: Name {
              loc: None,
              value: String::from("if"),
            },
            typ: Type::NonNull(NonNullType {
              loc: None,
              typ: Box::new(NullableType::Named(NamedType {
                loc: None,
                name: Name {
                  loc: None,
                  value: String::from("Boolean"),
                },
              })),
            }),
            default_value: None,
            directives: vec![],
          },
        ],
        locations: vec![
          Name {
            loc: None,
            value: String::from("FIELD"),
          },
          Name {
            loc: None,
            value: String::from("FRAGMENT_SPREAD"),
          },
          Name {
            loc: None,
            value: String::from("INLINE_FRAGMENT"),
          },
        ],
      })),
    ],
  }));
}
