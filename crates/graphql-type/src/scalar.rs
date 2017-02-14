/// A GraphQL scalar value. All scalars are [representable as strings][1], but
/// depending on the response format being used there may be a more appropriate
/// primitive for the given type.
///
/// This enum provides multiple different variants for the values that can be
/// used (mostly corresponding to JSON), but the [`ToString`][2] trait is
/// implemented so that values may always be seen in their string format.
///
/// Primitives are chosen based on the values of GraphQL scalars.
///
/// [1]: http://facebook.github.io/graphql/#sec-Scalars
/// [2]: https://doc.rust-lang.org/std/string/trait.ToString.html
pub enum Value {
  Null,
  Bool(bool),
  /// A signed 32-bit numeric non-fractional value. GraphQL integers will always
  /// be `i32`s and so we have a value for `i32`s. If you have a larger integer
  /// either use a float or a string.
  I32(i32),
  F64(f64),
  String(String),
}

// TODO: impl ToString for Value {
//   fn to_string(&self) -> String {
//     match self {
//       // &Null => String::from("null"),
//       // &Value::String(ref string) => string.to_string(),
//     }
//   }
// }

/// As expected by the name, a scalar represents a primitive value in GraphQL.
/// GraphQL responses take the form of a hierarchical tree; the leaves on these
/// trees are GraphQL scalars.
///
/// Defined in the [scalars][1] section of the specification.
///
/// This trait provides the result coercion and the input coercion functions
/// necessary to communicate an internal scalar value with the outside world.
///
/// [1]: http://facebook.github.io/graphql/#sec-Scalars
pub trait Scalar: Sized {
  /// Coerces a result to a GraphQL scalar value. Returns a `Value` if the
  /// serialization was successful. Returns `None` if the serialization was not
  /// successful. If `None` was returned then a field error will be thrown.
  fn serialize(self) -> Option<Value>;

  /// Coerces some GraphQL scalar value input to an internal value. If an
  /// internal value is returned then the deserialization was successful. If
  /// `None` was returned then the deserialization was unsuccessful.
  ///
  /// By default `None` is returned.
  fn deserialize(_: Value) -> Option<Self> { None }
}

/// The Int scalar type represents a signed 32‐bit numeric non‐fractional value.
/// Response formats that support a 32‐bit integer or a number type should use
/// that type to represent this scalar.
pub struct Int(i32);

// TODO: tests
impl Scalar for Int {
  /// Returns a `Value::I32` which wraps our value.
  fn serialize(self) -> Option<Value> {
    let Int(int) = self;
    Some(Value::I32(int))
  }

  /// Deserializes integers, floats that are in the range of an `i32`, and
  /// strings of the correct format.
  fn deserialize(value: Value) -> Option<Self> {
    match value {
      Value::Null => None,
      Value::Bool(_) => None,
      Value::I32(int) => Some(Int(int)),
      Value::F64(float) => {
        // Make sure the float is in-range then round to the nearest integer.
        // Otherwise just return `None`.
        if float < i32::max_value() as f64 && float > i32::min_value() as f64 {
          let int = if float < 0_f64 { float.ceil() } else { float.floor() };
          Some(Int(int as i32))
        } else {
          None
        }
      },
      Value::String(string) => i32::from_str_radix(&string, 10).ok().map(Int),
    }
  }
}

// TODO: Float
// TODO: String
// TODO: Boolean
// TODO: ID
