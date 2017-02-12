use super::{Position, Location};
use super::lexer::{Lexer, Error as LexerError, Token, TokenKind};
use super::ast;

/// Parses an iterator of characters into a GraphQL AST which may return an
/// error.
pub fn parse<I>(iter: I) -> Result<ast::Document, Error> where I: IntoIterator<Item=char> {
  let mut parser = Parser {
    lexer: Lexer::new(iter),
    include_location: true,
  };
  parser.parse_document()
}

/// Parses an iterator of characters into a GraphQL AST which may return an
/// error.
///
/// Will not include a location on the AST nodes which may drastically reduce
/// the size of the AST.
pub fn parse_without_location<I>(iter: I) -> Result<ast::Document, Error> where I: IntoIterator<Item=char> {
  let mut parser = Parser {
    lexer: Lexer::new(iter),
    include_location: false,
  };
  parser.parse_document()
}

/// A parser context.
struct Parser<I: Iterator<Item=char>> {
  /// The lexer that will be consumed while parsing.
  lexer: Lexer<I>,
  /// Whether or not to include the location in the AST object. Not including
  /// the location can drastically reduce the size of the AST.
  include_location: bool,
}

/// An error which occurred while parsing the source document into a GraphQL
/// AST.
#[derive(PartialEq, Debug)]
pub enum Error {
  /// The source ended unexpectedly.
  UnexpectedEnding(Position),
  /// When parsing we encountered an unexpected token.
  UnexpectedToken(Token),
  /// An error ocurred while lexing the source document.
  Lexer(LexerError),
  /// This error should never occur. But if it does, this cryptic error is
  /// better then panicking.
  Unreachable,
}

impl From<LexerError> for Error {
  fn from(error: LexerError) -> Error {
    Error::Lexer(error)
  }
}

impl<I> Parser<I> where I: Iterator<Item=char> {
  //////////////////////////////////////////////////////////////////////////////
  // Utilities
  //////////////////////////////////////////////////////////////////////////////

  /// Gets the current position of our parser.
  fn pos(&self) -> Position {
    self.lexer.pos()
  }

  /// Creates an optional location using starting and ending positions. The
  /// location may change depending on the options object.
  fn loc(&self, start: Position) -> Option<Location> {
    if self.include_location {
      Some(Location::new(start, self.pos()))
    } else {
      None
    }
  }

  /// Checks the next token kind without consuming it. Returns true if the
  /// token’s kind matches the provided kind, and returns false if the kind does
  /// not match or we are at the ending. If it encounters an error, false will
  /// be returned.
  pub fn check(&mut self, kind: TokenKind) -> bool {
    match self.lexer.peek() {
      Some(result) => match result {
        Ok(ref token) => token.kind == kind,
        Err(_) => false,
      },
      None => false,
    }
  }

  /// Checks if the next token is a name token with the provided name string.
  /// With this method we won’t need to allocate an owned string and can use
  /// string references.
  pub fn check_name(&mut self, name: &str) -> bool {
    match self.lexer.peek() {
      Some(result) => match result {
        Ok(ref token) => match token.kind {
          TokenKind::Name(ref kind_name) => kind_name == name,
          _ => false,
        },
        Err(_) => false,
      },
      None => false,
    }
  }

  /// Calls `next` on the lexer to consume the next token.
  fn next(&mut self) -> Option<Result<Token, Error>> {
    match self.lexer.next() {
      Some(Ok(token)) => Some(Ok(token)),
      Some(Err(error)) => Some(Err(Error::Lexer(error))),
      None => None,
    }
  }

  // Calls `next` if and only if the next token has the provided kind. If the
  // next token does not match the provided kind then `None` is returned.
  // Therefore `None` does not always mean the iterator has finished when
  // calling `next_if`! This method is useful with a pattern matched if
  // statement.
  pub fn next_if(&mut self, kind: TokenKind) -> Option<Result<Token, Error>> {
    if self.check(kind) {
      self.next()
    } else {
      None
    }
  }

  /// If the next token is a name token, we consume the token and return the
  /// name string. If the next token is not a name token, then we do not consume
  /// the next token.
  pub fn next_if_any_name(&mut self) -> Option<String> {
    match self.lexer.peek() {
      Some(Ok(ref token)) => match token.kind {
        TokenKind::Name(_) => (),
        _ => { return None },
      },
      _ => { return None },
    };
    match self.lexer.next() {
      Some(Ok(token)) => match token.kind {
        TokenKind::Name(name) => Some(name),
        _ => None,
      },
      _ => None,
    }
  }

  /// If the next token is a name token with the exact name provided then `next`
  // will be called. Otherwise `None` will be returned.
  pub fn next_if_name(&mut self, name: &str) -> bool {
    match self.lexer.peek() {
      Some(Ok(ref token)) => match token.kind {
        TokenKind::Name(ref name_ref) if name_ref == name => (),
        _ => { return false },
      },
      _ => { return false },
    };
    match self.lexer.next() {
      Some(Ok(token)) => match token.kind {
        TokenKind::Name(_) => true,
        _ => false,
      },
      _ => false,
    }
  }

  /// Generates an error that lets the user know we hit something unexpected in
  /// the lexer by consuming the next token in the lexer.
  fn unexpected(&mut self) -> Error {
    match self.next() {
      Some(Ok(token)) => Error::UnexpectedToken(token),
      Some(Err(error)) => error,
      None => Error::UnexpectedEnding(self.pos()),
    }
  }

  /// Asserts that the next token has the kind we expect. If the next token does
  /// not have that kind then we return an unexpected error.
  fn expect(&mut self, kind: TokenKind) -> Result<(), Error> {
    match self.next() {
      Some(Ok(ref token)) if token.kind == kind => Ok(()),
      Some(Ok(token)) => Err(Error::UnexpectedToken(token)),
      Some(Err(error)) => Err(error),
      None => Err(Error::UnexpectedEnding(self.pos())),
    }
  }

  /// Asserts that the next token is a name token with the name that we expect.
  /// If the next token does not have that name then we return an unexpected
  /// error.
  fn expect_name(&mut self, name: &str) -> Result<(), Error> {
    match self.next() {
      Some(Ok(Token { kind: TokenKind::Name(ref token_name), .. })) if token_name == name => Ok(()),
      Some(Ok(token)) => Err(Error::UnexpectedToken(token)),
      Some(Err(error)) => Err(error),
      None => Err(Error::UnexpectedEnding(self.pos())),
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Name
  //////////////////////////////////////////////////////////////////////////////

  /// Converts a name token into a name parse node.
  fn parse_name(&mut self) -> Result<ast::Name, Error> {
    let start = self.pos();

    match self.next_if_any_name() {
      Some(name) => Ok(ast::Name {
        loc: self.loc(start),
        value: name,
      }),
      _ => Err(self.unexpected()),
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Document
  //////////////////////////////////////////////////////////////////////////////

  /// ```txt
  /// Document : Definition+
  /// ```
  fn parse_document(&mut self) -> Result<ast::Document, Error> {
    let start = self.pos();
    let mut definitions = Vec::new();

    while self.lexer.peek() != None {
      definitions.push(try!(self.parse_definition()));
    }

    // Return an error if we have no definitions.
    if definitions.len() < 1 {
      return Err(Error::UnexpectedEnding(self.pos()));
    }

    Ok(ast::Document {
      loc: self.loc(start),
      definitions: definitions,
    })
  }

  /// ```txt
  /// Definition :
  ///   - OperationDefinition
  ///   - FragmentDefinition
  ///   - TypeSystemDefinition
  /// ```
  fn parse_definition(&mut self) -> Result<ast::Definition, Error> {
    if {
      self.check(TokenKind::LeftBrace) ||
      self.check_name("query") ||
      self.check_name("mutation") ||
      self.check_name("subscription")
    } {
      self.parse_operation_definition().map(ast::Definition::Operation)
    }
    else if self.check_name("fragment") {
      self.parse_fragment_definition().map(ast::Definition::Fragment)
    }
    else if self.check_name("schema") {
      unimplemented!();
    }
    else if self.check_name("scalar") {
      unimplemented!();
    }
    else if self.check_name("type") {
      unimplemented!();
    }
    else if self.check_name("interface") {
      unimplemented!();
    }
    else if self.check_name("union") {
      unimplemented!();
    }
    else if self.check_name("enum") {
      unimplemented!();
    }
    else if self.check_name("input") {
      unimplemented!();
    }
    else if self.check_name("extend") {
      unimplemented!();
    }
    else if self.check_name("directive") {
      unimplemented!();
    }
    else {
      Err(self.unexpected())
    }
  }

  /// ```txt
  /// OperationDefinition :
  ///   - SelectionSet
  ///   - OperationType Name? VariableDefinitions? Directives? SelectionSet
  /// ```
  fn parse_operation_definition(&mut self) -> Result<ast::OperationDefinition, Error> {
    let start = self.pos();
    let mut operation = ast::OperationType::Query;
    let mut name: Option<ast::Name> = None;
    let mut variable_definitions: Vec<ast::VariableDefinition> = vec![];
    let mut directives: Vec<ast::Directive> = vec![];

    if !self.check(TokenKind::LeftBrace) {
      operation = try!(self.parse_operation_type());

      {
        let start = self.pos();
        if let Some(name_string) = self.next_if_any_name() {
          name = Some(ast::Name {
            loc: self.loc(start),
            value: name_string,
          });
        }
      }

      variable_definitions = if self.check(TokenKind::LeftParen) { try!(self.parse_variable_definitions()) } else { vec![] };
      directives = try!(self.parse_directives());
    }

    let selection_set = try!(self.parse_selection_set());

    Ok(ast::OperationDefinition {
      loc: self.loc(start),
      operation: operation,
      name: name,
      variable_definitions: variable_definitions,
      directives: directives,
      selection_set: selection_set,
    })
  }

  /// ```txt
  /// OperationType : one of query mutation subscription
  /// ```
  fn parse_operation_type(&mut self) -> Result<ast::OperationType, Error> {
    match self.next_if_any_name().as_ref().map(|x| &**x) {
      Some("query") => Ok(ast::OperationType::Query),
      Some("mutation") => Ok(ast::OperationType::Mutation),
      Some("subscription") => Ok(ast::OperationType::Subscription),
      _ => Err(self.unexpected()),
    }
  }

  /// ```txt
  /// VariableDefinitions : ( VariableDefinition+ )
  /// ```
  fn parse_variable_definitions(&mut self) -> Result<Vec<ast::VariableDefinition>, Error> {
    if let Some(_) = self.next_if(TokenKind::LeftParen) {
      let mut variable_definitions = vec![];

      loop {
        if let Some(token) = self.next_if(TokenKind::RightParen) {
          // If there were no variable definitions then we need to return an
          // error.
          if variable_definitions.len() < 1 {
            return Err(Error::UnexpectedToken(try!(token)));
          }
          break;
        }
        variable_definitions.push(try!(self.parse_variable_definition()));
      }

      Ok(variable_definitions)
    } else {
      Err(self.unexpected())
    }
  }

  /// ```txt
  /// VariableDefinition : Variable : Type DefaultValue?
  /// ```
  fn parse_variable_definition(&mut self) -> Result<ast::VariableDefinition, Error> {
    let start = self.pos();

    let variable = try!(self.parse_variable());

    try!(self.expect(TokenKind::Colon));

    let typ = try!(self.parse_type_reference());

    let default_value = {
      if let Some(_) = self.next_if(TokenKind::Equals) {
        Some(try!(self.parse_value_literal()))
      } else {
        None
      }
    };

    Ok(ast::VariableDefinition {
      loc: self.loc(start),
      variable: variable,
      typ: typ,
      default_value: default_value,
    })
  }

  /// ```txt
  /// Variable : $ Name
  /// ```
  fn parse_variable(&mut self) -> Result<ast::Variable, Error> {
    let start = self.pos();

    try!(self.expect(TokenKind::Dollar));

    let name = try!(self.parse_name());

    Ok(ast::Variable {
      loc: self.loc(start),
      name: name,
    })
  }

  /// ```txt
  /// SelectionSet : { Selection+ }
  /// ```
  fn parse_selection_set(&mut self) -> Result<ast::SelectionSet, Error> {
    let start = self.pos();
    let mut selections: Vec<ast::Selection> = vec![];

    try!(self.expect(TokenKind::LeftBrace));

    loop {
      if let Some(token) = self.next_if(TokenKind::RightBrace) {
        // If there were no selections then we need to return an error.
        if selections.len() < 1 {
          return Err(Error::UnexpectedToken(try!(token)));
        }
        break;
      }
      selections.push(try!(self.parse_selection()));
    }

    Ok(ast::SelectionSet {
      loc: self.loc(start),
      selections: selections,
    })
  }

  /// ```txt
  /// Selection :
  ///   - Field
  ///   - FragmentSpread
  ///   - InlineFragment
  /// ```
  fn parse_selection(&mut self) -> Result<ast::Selection, Error> {
    if self.check(TokenKind::Ellipsis) {
      self.parse_fragment()
    } else {
      self.parse_field().map(ast::Selection::Field)
    }
  }

  /// ```txt
  /// Field : Alias? Name Arguments? Directives? SelectionSet?
  ///
  /// Alias : Name :
  /// ```
  fn parse_field(&mut self) -> Result<ast::Field, Error> {
    let start = self.pos();

    let mut alias: Option<ast::Name> = None;
    let mut name = try!(self.parse_name());

    if let Some(_) = self.next_if(TokenKind::Colon) {
      alias = Some(name);
      name = try!(self.parse_name());
    }

    let arguments = if self.check(TokenKind::LeftParen) { try!(self.parse_arguments()) } else { vec![] };
    let directives = try!(self.parse_directives());

    let selection_set = {
      if self.check(TokenKind::LeftBrace) {
        Some(try!(self.parse_selection_set()))
      } else {
        None
      }
    };

    Ok(ast::Field {
      loc: self.loc(start),
      alias: alias,
      name: name,
      arguments: arguments,
      directives: directives,
      selection_set: selection_set,
    })
  }

  /// ```txt
  /// Arguments : ( Argument + )
  /// ```
  fn parse_arguments(&mut self) -> Result<Vec<ast::Argument>, Error> {
    let mut arguments: Vec<ast::Argument> = vec![];
    try!(self.expect(TokenKind::LeftParen));
    loop {
      if let Some(token) = self.next_if(TokenKind::RightParen) {
        if arguments.len() < 1 {
          return Err(Error::UnexpectedToken(try!(token)));
        }
        break;
      }
      arguments.push(try!(self.parse_argument()));
    }
    Ok(arguments)
  }

  /// ```txt
  /// Argument : Name : Value
  /// ```
  fn parse_argument(&mut self) -> Result<ast::Argument, Error> {
    let start = self.pos();
    let name = try!(self.parse_name());
    try!(self.expect(TokenKind::Colon));
    let value = try!(self.parse_value_literal());
    Ok(ast::Argument {
      loc: self.loc(start),
      name: name,
      value: value,
    })
  }

  //////////////////////////////////////////////////////////////////////////////
  // Fragments
  //////////////////////////////////////////////////////////////////////////////

  /// Parses both `FragmentSpread` and `InlineFragment`.
  ///
  /// ```txt
  /// FragmentSpread : ... FragmentName Directives?
  /// ```
  ///
  /// ```txt
  /// InlineFragment : ... TypeCondition? Directives? SelectionSet
  /// ```
  fn parse_fragment(&mut self) -> Result<ast::Selection, Error> {
    let start = self.pos();
    try!(self.expect(TokenKind::Ellipsis));
    let start_after_ellipsis = self.pos();

    // If there is a name to eat then this is a fragment spread.
    if !self.check_name("on") {
      if let Some(name_value) = self.next_if_any_name() {
        let name = ast::Name {
          loc: self.loc(start_after_ellipsis),
          value: name_value,
        };
        let directives = try!(self.parse_directives());
        return Ok(ast::Selection::FragmentSpread(ast::FragmentSpread {
          loc: self.loc(start),
          name: name,
          directives: directives,
        }));
      }
    }

    let mut type_condition: Option<ast::NamedType> = None;

    // If we see `on` then we know there will be a type condition and so we parse
    // a named type.
    if self.next_if_name("on") {
      type_condition = Some(try!(self.parse_named_type()));
    }

    let directives = try!(self.parse_directives());
    let selection_set = try!(self.parse_selection_set());

    Ok(ast::Selection::InlineFragment(ast::InlineFragment {
      loc: self.loc(start),
      type_condition: type_condition,
      directives: directives,
      selection_set: selection_set,
    }))
  }

  /// ```txt
  /// FragmentDefinition :
  ///  - fragment FragmentName on TypeCondition Directives? SelectionSet
  ///
  /// TypeCondition : NamedType
  /// ```
  fn parse_fragment_definition(&mut self) -> Result<ast::FragmentDefinition, Error> {
    let start = self.pos();
    try!(self.expect_name("fragment"));
    let name = try!(self.parse_fragment_name());
    try!(self.expect_name("on"));
    let type_condition = try!(self.parse_named_type());
    let directives = try!(self.parse_directives());
    let selection_set = try!(self.parse_selection_set());
    Ok(ast::FragmentDefinition {
      loc: self.loc(start),
      name: name,
      type_condition: type_condition,
      directives: directives,
      selection_set: selection_set,
    })
  }

  /// ```txt
  /// FragmentName : Name but not `on`
  /// ```
  fn parse_fragment_name(&mut self) -> Result<ast::Name, Error> {
    if self.check_name("ok") {
      Err(self.unexpected())
    } else {
      self.parse_name()
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  // Values
  //////////////////////////////////////////////////////////////////////////////

  /// ```txt
  /// Value[Const] :
  ///   - [~Const] Variable
  ///   - IntValue
  ///   - FloatValue
  ///   - StringValue
  ///   - BooleanValue
  ///   - NullValue
  ///   - EnumValue
  ///   - ListValue[?Const]
  ///   - ObjectValue[?Const]
  ///
  /// BooleanValue : one of `true` `false`
  ///
  /// NullValue : `null`
  ///
  /// EnumValue : Name but not `true`, `false` or `null`
  /// ```
  fn parse_value_literal(&mut self) -> Result<ast::Value, Error> {
    let start = self.pos();

    if self.check(TokenKind::LeftBracket) {
      Ok(ast::Value::List(try!(self.parse_list())))
    }
    else if self.check(TokenKind::LeftBrace) {
      Ok(ast::Value::Object(try!(self.parse_object())))
    }
    else if self.check_name("null") {
      self.next();
      Ok(ast::Value::Null(ast::NullValue {
        loc: self.loc(start),
      }))
    }
    else if self.check_name("true") {
      self.next();
      Ok(ast::Value::Boolean(ast::BooleanValue {
        loc: self.loc(start),
        value: true,
      }))
    }
    else if self.check_name("false") {
      self.next();
      Ok(ast::Value::Boolean(ast::BooleanValue {
        loc: self.loc(start),
        value: false,
      }))
    }
    else if self.check(TokenKind::Dollar) {
      Ok(ast::Value::Variable(try!(self.parse_variable())))
    }
    else {
      match self.next() {
        Some(Ok(token)) => match token.kind {
          TokenKind::Int(integer) => Ok(ast::Value::Int(ast::IntValue {
            loc: self.loc(start),
            value: integer,
          })),
          TokenKind::Float(float) => Ok(ast::Value::Float(ast::FloatValue {
            loc: self.loc(start),
            value: float,
          })),
          TokenKind::String(string) => Ok(ast::Value::String(ast::StringValue {
            loc: self.loc(start),
            value: string,
          })),
          TokenKind::Name(name) => Ok(ast::Value::Enum(ast::EnumValue {
            loc: self.loc(start),
            value: name,
          })),
          _ => Err(Error::UnexpectedToken(token)),
        },
        Some(Err(error)) => Err(error),
        None => Err(Error::UnexpectedEnding(self.pos())),
      }
    }
  }

  /// ```txt
  /// ListValue[Const] :
  ///  - [ ]
  ///  - [ Value[?Const]+ ]
  /// ```
  fn parse_list(&mut self) -> Result<ast::ListValue, Error> {
    let start = self.pos();
    let mut values: Vec<ast::Value> = vec![];
    try!(self.expect(TokenKind::LeftBracket));
    loop {
      if self.next_if(TokenKind::RightBracket).is_some() {
        break;
      }
      values.push(try!(self.parse_value_literal()));
    }
    Ok(ast::ListValue {
      loc: self.loc(start),
      values: values,
    })
  }

  /// ```txt
  /// ObjectValue[Const] :
  ///  - { }
  ///  - { ObjectField[?Const]+ }
  /// ```
  fn parse_object(&mut self) -> Result<ast::ObjectValue, Error> {
    let start = self.pos();
    let mut fields: Vec<ast::ObjectField> = vec![];
    try!(self.expect(TokenKind::LeftBrace));
    loop {
      if self.next_if(TokenKind::RightBrace).is_some() {
        break;
      }
      fields.push(try!(self.parse_object_field()));
    }
    Ok(ast::ObjectValue {
      loc: self.loc(start),
      fields: fields,
    })
  }

  /// ```txt
  /// ObjectField[Const] : Name : Value[?Const]
  /// ```
  fn parse_object_field(&mut self) -> Result<ast::ObjectField, Error> {
    let start = self.pos();
    let name = try!(self.parse_name());
    try!(self.expect(TokenKind::Colon));
    let value = try!(self.parse_value_literal());
    Ok(ast::ObjectField {
      loc: self.loc(start),
      name: name,
      value: value,
    })
  }

  //////////////////////////////////////////////////////////////////////////////
  // Directives
  //////////////////////////////////////////////////////////////////////////////

  /// If there are no directives this function will return an empty array. This
  /// diverges from the spec. The spec uses the definition `Directive+`, but we
  /// change it to `Directive*`.
  ///
  /// ```txt
  /// Directives : Directive*
  /// ```
  fn parse_directives(&mut self) -> Result<Vec<ast::Directive>, Error> {
    let mut directives: Vec<ast::Directive> = vec![];
    loop {
      if !self.check(TokenKind::At) {
        break;
      }
      directives.push(try!(self.parse_directive()));
    }
    Ok(directives)
  }

  /// ```txt
  /// Directive : @ Name Arguments?
  /// ```
  fn parse_directive(&mut self) -> Result<ast::Directive, Error> {
    let start = self.pos();
    try!(self.expect(TokenKind::At));
    let name = try!(self.parse_name());
    let arguments = if self.check(TokenKind::LeftParen) { try!(self.parse_arguments()) } else { vec![] };
    Ok(ast::Directive {
      loc: self.loc(start),
      name: name,
      arguments: arguments,
    })
  }

  //////////////////////////////////////////////////////////////////////////////
  // Type Reference
  //////////////////////////////////////////////////////////////////////////////

  /// ```txt
  /// Type :
  ///   - NamedType
  ///   - ListType
  ///   - NonNullType
  /// ```
  fn parse_type_reference(&mut self) -> Result<ast::Type, Error> {
    let start = self.pos();

    let nullable_type = {
      // If we hit a left bracket, this is likely an array type.
      if let Some(_) = self.next_if(TokenKind::LeftBracket) {
        let typ = try!(self.parse_type_reference());
        try!(self.expect(TokenKind::RightBracket));
        ast::NullableType::List(ast::ListType {
          loc: self.loc(start),
          typ: Box::new(typ),
        })
      }
      // Otherwise we try and parse a named type.
      else {
        ast::NullableType::Named(try!(self.parse_named_type()))
      }
    };

    // If we have a bang at the end, this is a non-null type.
    if let Some(_) = self.next_if(TokenKind::Bang) {
      Ok(ast::Type::NonNull(ast::NonNullType {
        loc: self.loc(start),
        typ: Box::new(nullable_type),
      }))
    }
    // Otherwise, convert the nullable type into an actual type.
    else {
      Ok(From::from(nullable_type))
    }
  }

  /// ```txt
  /// NamedType : Name
  /// ```
  fn parse_named_type(&mut self) -> Result<ast::NamedType, Error> {
    let start = self.pos();
    let name = try!(self.parse_name());
    Ok(ast::NamedType {
      loc: self.loc(start),
      name: name,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! assert_parse_error {
    ($source:expr, $error:expr) => ({
      assert_eq!(parse($source.chars()), Err($error));
    })
  }

  /// Creates a `Position` value for a one line situation as is common in our
  /// tests. Takes the 0-indexed value and generates the column and line
  /// numbers.
  fn pos1(index: usize) -> Position {
    Position {
      index: index,
      line: 1,
      column: index + 1,
    }
  }

  #[test]
  fn test_document_empty() {
    assert_parse_error!("", Error::UnexpectedEnding(pos1(0)));
    assert_parse_error!(" ", Error::UnexpectedEnding(pos1(0)));
    assert_parse_error!("  ", Error::UnexpectedEnding(pos1(1)));
    assert_parse_error!("   ", Error::UnexpectedEnding(pos1(2)));
  }

  #[test]
  fn test_selection_set_empty() {
    assert_parse_error!("{}", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(1), pos1(1))));
    assert_parse_error!("{ }", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(2), pos1(2))));
    assert_parse_error!("{  }", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(3), pos1(3))));
    assert_parse_error!("{   }", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(4), pos1(4))));
    assert_parse_error!("{", Error::UnexpectedEnding(pos1(0)));
    assert_parse_error!("{  ", Error::UnexpectedEnding(pos1(2)));
    assert_parse_error!("query {}", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(7), pos1(7))));
    assert_parse_error!("mutation {}", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(10), pos1(10))));
    assert_parse_error!("subscription {}", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(14), pos1(14))));
    assert_parse_error!("{ foo {} }", Error::UnexpectedToken(Token::new(TokenKind::RightBrace, pos1(7), pos1(7))));
  }

  #[test]
  fn parse_definition_bad_name() {
    assert_parse_error!("hello", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("hello")), pos1(0), pos1(4))));
    assert_parse_error!("hello {}", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("hello")), pos1(0), pos1(4))));
    assert_parse_error!("  world", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("world")), pos1(2), pos1(6))));
    assert_parse_error!("foo  ", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("foo")), pos1(0), pos1(2))));
    assert_parse_error!("  bar  ", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("bar")), pos1(2), pos1(4))));
  }

  #[test]
  fn test_variable_definitions_empty() {
    assert_parse_error!("query ()", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(7), pos1(7))));
    assert_parse_error!("mutation ()", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(10), pos1(10))));
    assert_parse_error!("subscription ()", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(14), pos1(14))));
    assert_parse_error!("query (  )", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(9), pos1(9))));
  }

  #[test]
  fn test_variable_definitions_no_dollar() {
    assert_parse_error!("query (foo)", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("foo")), pos1(7), pos1(9))));
    assert_parse_error!("query ($foo: Foo, bar)", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("bar")), pos1(18), pos1(20))));
  }

  #[test]
  fn test_variable_definitions_no_colon() {
    assert_parse_error!("query ($foo Foo)", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("Foo")), pos1(12), pos1(14))));
    assert_parse_error!("query ($foo: Foo, $bar Bar)", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("Bar")), pos1(23), pos1(25))));
  }

  #[test]
  fn test_variable_definitions_no_type_reference() {
    assert_parse_error!("query ($foo:, $bar)", Error::UnexpectedToken(Token::new(TokenKind::Dollar, pos1(14), pos1(14))));
  }

  #[test]
  fn test_variable_definitions_no_value_for_default_value() {
    assert_parse_error!("query ($foo: Foo =)", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(18), pos1(18))));
  }

  #[test]
  fn test_arguments_empty() {
    assert_parse_error!("{ foo() }", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(6), pos1(6))));
    assert_parse_error!("{ foo( ) }", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(7), pos1(7))));
    assert_parse_error!("{ foo(  ) }", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(8), pos1(8))));
    assert_parse_error!("{ foo(   ) }", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(9), pos1(9))));
    assert_parse_error!("{ foo { bar() } }", Error::UnexpectedToken(Token::new(TokenKind::RightParen, pos1(12), pos1(12))));
  }

  #[test]
  fn test_arguments_no_colon() {
    assert_parse_error!("{ foo(arg1 12) }", Error::UnexpectedToken(Token::new(TokenKind::Int(12), pos1(11), pos1(12))));
    assert_parse_error!("{ foo(arg1 arg2) }", Error::UnexpectedToken(Token::new(TokenKind::Name(String::from("arg2")), pos1(11), pos1(14))));
    assert_parse_error!("{ foo(arg1: 12, arg2 34) }", Error::UnexpectedToken(Token::new(TokenKind::Int(34), pos1(21), pos1(22))));
  }
}
