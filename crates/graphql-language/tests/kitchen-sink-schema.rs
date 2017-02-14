#![cfg(feature = "type_system")]

extern crate graphql_language;

use graphql_language::{parse_without_location, print};
use graphql_language::ast::*;

const SOURCE: &'static str =
r#"schema {
  query: QueryType
  mutation: MutationType
}

type Foo implements Bar {
  one: Type
  two(argument: InputType!): Type
  three(argument: InputType, other: String): Int
  four(argument: String = "string"): String
  five(argument: [String] = ["string", "string"]): String
  six(argument: InputType = {key: "value"}): Type
  seven(argument: Int = null): Type
}

type AnnotatedObject @onObject(arg: "value") {
  annotatedField(arg: Type = "default" @onArg): Type @onField
}

interface Bar {
  one: Type
  four(argument: String = "string"): String
}

interface AnnotatedInterface @onInterface {
  annotatedField(arg: Type @onArg): Type @onField
}

union Feed = Story | Article | Advert

union AnnotatedUnion @onUnion = A | B

scalar CustomScalar

scalar AnnotatedScalar @onScalar

enum Site {
  DESKTOP
  MOBILE
}

enum AnnotatedEnum @onEnum {
  ANNOTATED_VALUE @onEnumValue
  OTHER_VALUE
}

input InputType {
  key: String!
  answer: Int = 42
}

input AnnotatedInput @onInputObjectType {
  annotatedField: Type @onField
}

extend type Foo {
  seven(argument: [String]): Type
}

extend type Foo @onType {}

type NoFields {}

directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT
"#;

#[test]
fn kitchen_sink_schema_parse_and_print_2x() {
  let parse1 = parse_without_location(SOURCE.chars()).unwrap();
  let print1 = print(&parse1);
  let parse2 = parse_without_location(print1.chars()).unwrap();
  let print2 = print(&parse2);

  assert_eq!(parse1, parse2);
  assert_eq!(print1, print2);
}

#[test]
fn kitchen_sink_schema_ast() {
  let document = Document {
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
  };
  assert_eq!(parse_without_location(SOURCE.chars()).as_ref(), Ok(&document));
  assert_eq!(print(&document), SOURCE);
}
