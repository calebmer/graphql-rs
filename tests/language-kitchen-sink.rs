extern crate graphql;

use std::io::prelude::*;
use std::fs::File;
use graphql::language::parse_without_location;
use graphql::language::ast::*;

#[test]
fn parse_kitchen_sink() {
  let mut file = File::open("tests/fixtures/kitchen-sink.graphql").unwrap();
  let mut string = String::new();
  file.read_to_string(&mut string).unwrap();
  let ast = parse_without_location(string.chars());
  assert_eq!(ast, Ok(Document {
    loc: None,
    definitions: vec![
      Definition::Operation(OperationDefinition {
        loc: None,
        operation: OperationType::Query,
        name: Some(Name {
          loc: None,
          value: String::from("queryName"),
        }),
        variable_definitions: vec![
          VariableDefinition {
            loc: None,
            variable: Variable {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("foo"),
              },
            },
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("ComplexType"),
              },
            }),
            default_value: None,
          },
          VariableDefinition {
            loc: None,
            variable: Variable {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("site"),
              },
            },
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("Site"),
              },
            }),
            default_value: Some(Value::Enum(EnumValue {
              loc: None,
              value: String::from("MOBILE"),
            })),
          },
        ],
        directives: vec![],
        selection_set: SelectionSet {
          loc: None,
          selections: vec![
            Selection::Field(Field {
              loc: None,
              alias: Some(Name {
                loc: None,
                value: String::from("whoever123is"),
              }),
              name: Name {
                loc: None,
                value: String::from("node"),
              },
              arguments: vec![
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("id"),
                  },
                  value: Value::List(ListValue {
                    loc: None,
                    values: vec![
                      Value::Int(IntValue {
                        loc: None,
                        value: 123,
                      }),
                      Value::Int(IntValue {
                        loc: None,
                        value: 456,
                      }),
                    ],
                  }),
                },
              ],
              directives: vec![],
              selection_set: Some(SelectionSet {
                loc: None,
                selections: vec![
                  Selection::Field(Field {
                    loc: None,
                    alias: None,
                    name: Name {
                      loc: None,
                      value: String::from("id"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selection_set: None,
                  }),
                  Selection::FragmentSpread(FragmentSpread {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("on"),
                    },
                    directives: vec![],
                  }),
                  Selection::Field(Field {
                    loc: None,
                    alias: None,
                    name: Name {
                      loc: None,
                      value: String::from("User"),
                    },
                    arguments: vec![],
                    directives: vec![
                      Directive {
                        loc: None,
                        name: Name {
                          loc: None,
                          value: String::from("defer"),
                        },
                        arguments: vec![],
                      },
                    ],
                    selection_set: Some(SelectionSet {
                      loc: None,
                      selections: vec![
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("field2"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: Some(SelectionSet {
                            loc: None,
                            selections: vec![
                              Selection::Field(Field {
                                loc: None,
                                alias: None,
                                name: Name {
                                  loc: None,
                                  value: String::from("id"),
                                },
                                arguments: vec![],
                                directives: vec![],
                                selection_set: None,
                              }),
                              Selection::Field(Field {
                                loc: None,
                                alias: Some(Name {
                                  loc: None,
                                  value: String::from("alias"),
                                }),
                                name: Name {
                                  loc: None,
                                  value: String::from("field1"),
                                },
                                arguments: vec![
                                  Argument {
                                    loc: None,
                                    name: Name {
                                      loc: None,
                                      value: String::from("first"),
                                    },
                                    value: Value::Int(IntValue {
                                      loc: None,
                                      value: 10,
                                    }),
                                  },
                                  Argument {
                                    loc: None,
                                    name: Name {
                                      loc: None,
                                      value: String::from("after"),
                                    },
                                    value: Value::Variable(Variable {
                                      loc: None,
                                      name: Name {
                                        loc: None,
                                        value: String::from("foo"),
                                      },
                                    }),
                                  },
                                ],
                                directives: vec![
                                  Directive {
                                    loc: None,
                                    name: Name {
                                      loc: None,
                                      value: String::from("include"),
                                    },
                                    arguments: vec![
                                      Argument {
                                        loc: None,
                                        name: Name {
                                          loc: None,
                                          value: String::from("if"),
                                        },
                                        value: Value::Variable(Variable {
                                          loc: None,
                                          name: Name {
                                            loc: None,
                                            value: String::from("foo"),
                                          },
                                        }),
                                      },
                                    ],
                                  },
                                ],
                                selection_set: Some(SelectionSet {
                                  loc: None,
                                  selections: vec![
                                    Selection::Field(Field {
                                      loc: None,
                                      alias: None,
                                      name: Name {
                                        loc: None,
                                        value: String::from("id"),
                                      },
                                      arguments: vec![],
                                      directives: vec![],
                                      selection_set: None,
                                    }),
                                    Selection::FragmentSpread(FragmentSpread {
                                      loc: None,
                                      name: Name {
                                        loc: None,
                                        value: String::from("frag"),
                                      },
                                      directives: vec![],
                                    }),
                                  ],
                                }),
                              }),
                            ],
                          }),
                        }),
                      ],
                    }),
                  }),
                  Selection::InlineFragment(InlineFragment {
                    loc: None,
                    type_condition: None,
                    directives: vec![
                      Directive {
                        loc: None,
                        name: Name {
                          loc: None,
                          value: String::from("skip"),
                        },
                        arguments: vec![
                          Argument {
                            loc: None,
                            name: Name {
                              loc: None,
                              value: String::from("unless"),
                            },
                            value: Value::Variable(Variable {
                              loc: None,
                              name: Name {
                                loc: None,
                                value: String::from("foo"),
                              },
                            }),
                          },
                        ],
                      },
                    ],
                    selection_set: SelectionSet {
                      loc: None,
                      selections: vec![
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("id"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: None,
                        }),
                      ],
                    },
                  }),
                  Selection::InlineFragment(InlineFragment {
                    loc: None,
                    type_condition: None,
                    directives: vec![],
                    selection_set: SelectionSet {
                      loc: None,
                      selections: vec![
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("id"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: None,
                        }),
                      ],
                    },
                  }),
                ],
              }),
            }),
          ],
        },
      }),
      Definition::Operation(OperationDefinition {
        loc: None,
        operation: OperationType::Mutation,
        name: Some(Name {
          loc: None,
          value: String::from("likeStory"),
        }),
        variable_definitions: vec![],
        directives: vec![],
        selection_set: SelectionSet {
          loc: None,
          selections: vec![
            Selection::Field(Field {
              loc: None,
              alias: None,
              name: Name {
                loc: None,
                value: String::from("like"),
              },
              arguments: vec![
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("story"),
                  },
                  value: Value::Int(IntValue {
                    loc: None,
                    value: 123,
                  }),
                },
              ],
              directives: vec![
                Directive {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("defer"),
                  },
                  arguments: vec![],
                },
              ],
              selection_set: Some(SelectionSet {
                loc: None,
                selections: vec![
                  Selection::Field(Field {
                    loc: None,
                    alias: None,
                    name: Name {
                      loc: None,
                      value: String::from("story"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selection_set: Some(SelectionSet {
                      loc: None,
                      selections: vec![
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("id"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: None,
                        }),
                      ],
                    }),
                  }),
                ],
              }),
            }),
          ],
        },
      }),
      Definition::Operation(OperationDefinition {
        loc: None,
        operation: OperationType::Subscription,
        name: Some(Name {
          loc: None,
          value: String::from("StoryLikeSubscription"),
        }),
        variable_definitions: vec![
          VariableDefinition {
            loc: None,
            variable: Variable {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("input"),
              },
            },
            typ: Type::Named(NamedType {
              loc: None,
              name: Name {
                loc: None,
                value: String::from("StoryLikeSubscribeInput"),
              },
            }),
            default_value: None,
          },
        ],
        directives: vec![],
        selection_set: SelectionSet {
          loc: None,
          selections: vec![
            Selection::Field(Field {
              loc: None,
              alias: None,
              name: Name {
                loc: None,
                value: String::from("storyLikeSubscribe"),
              },
              arguments: vec![
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("input"),
                  },
                  value: Value::Variable(Variable {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("input"),
                    },
                  }),
                },
              ],
              directives: vec![],
              selection_set: Some(SelectionSet {
                loc: None,
                selections: vec![
                  Selection::Field(Field {
                    loc: None,
                    alias: None,
                    name: Name {
                      loc: None,
                      value: String::from("story"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selection_set: Some(SelectionSet {
                      loc: None,
                      selections: vec![
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("likers"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: Some(SelectionSet {
                            loc: None,
                            selections: vec![
                              Selection::Field(Field {
                                loc: None,
                                alias: None,
                                name: Name {
                                  loc: None,
                                  value: String::from("count"),
                                },
                                arguments: vec![],
                                directives: vec![],
                                selection_set: None,
                              }),
                            ],
                          }),
                        }),
                        Selection::Field(Field {
                          loc: None,
                          alias: None,
                          name: Name {
                            loc: None,
                            value: String::from("likeSentence"),
                          },
                          arguments: vec![],
                          directives: vec![],
                          selection_set: Some(SelectionSet {
                            loc: None,
                            selections: vec![
                              Selection::Field(Field {
                                loc: None,
                                alias: None,
                                name: Name {
                                  loc: None,
                                  value: String::from("text"),
                                },
                                arguments: vec![],
                                directives: vec![],
                                selection_set: None,
                              }),
                            ],
                          }),
                        }),
                      ],
                    }),
                  }),
                ],
              }),
            }),
          ],
        },
      }),
      Definition::Fragment(FragmentDefinition {
        loc: None,
        name: Name {
          loc: None,
          value: String::from("frag"),
        },
        type_condition: NamedType {
          loc: None,
          name: Name {
            loc: None,
            value: String::from("Friend"),
          },
        },
        directives: vec![],
        selection_set: SelectionSet {
          loc: None,
          selections: vec![
            Selection::Field(Field {
              loc: None,
              alias: None,
              name: Name {
                loc: None,
                value: String::from("foo"),
              },
              arguments: vec![
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("size"),
                  },
                  value: Value::Variable(Variable {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("size"),
                    },
                  }),
                },
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("bar"),
                  },
                  value: Value::Variable(Variable {
                    loc: None,
                    name: Name {
                      loc: None,
                      value: String::from("b"),
                    },
                  }),
                },
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("obj"),
                  },
                  value: Value::Object(ObjectValue {
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
                  }),
                },
              ],
              directives: vec![],
              selection_set: None,
            }),
          ],
        },
      }),
      Definition::Operation(OperationDefinition {
        loc: None,
        operation: OperationType::Query,
        name: None,
        variable_definitions: vec![],
        directives: vec![],
        selection_set: SelectionSet {
          loc: None,
          selections: vec![
            Selection::Field(Field {
              loc: None,
              alias: None,
              name: Name {
                loc: None,
                value: String::from("unnamed"),
              },
              arguments: vec![
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("truthy"),
                  },
                  value: Value::Boolean(BooleanValue {
                    loc: None,
                    value: true,
                  }),
                },
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("falsey"),
                  },
                  value: Value::Boolean(BooleanValue {
                    loc: None,
                    value: false,
                  }),
                },
                Argument {
                  loc: None,
                  name: Name {
                    loc: None,
                    value: String::from("nullish"),
                  },
                  value: Value::Null(NullValue {
                    loc: None,
                  }),
                },
              ],
              directives: vec![],
              selection_set: None,
            }),
            Selection::Field(Field {
              loc: None,
              alias: None,
              name: Name {
                loc: None,
                value: String::from("query"),
              },
              arguments: vec![],
              directives: vec![],
              selection_set: None,
            }),
          ],
        },
      }),
    ],
  }));
}
