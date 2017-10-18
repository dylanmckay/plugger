pub type TypeName = &'static str;

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: TypeName,
}

#[derive(Clone)]
pub struct Method
{
    /// A pointer to the function.
    pub method_pointer: *mut fn(),
    /// Marshall functions for every supported language.
    // NOTE: This is a vec and not a hash map because it's easier to construct
    // a Vec inside a syntax extension.
    pub lang_marshalls: Vec<(&'static str, *mut fn())>,

    /// The name of the method.
    pub name: &'static str,
    /// The parameter list.
    pub parameters: Vec<Parameter>,
    /// The return type (if any).
    pub ret: Option<TypeName>,

    /// Whether the method has a receiver.
    pub is_static: bool,
}

#[derive(Clone)]
pub struct Field
{
    /// The number of bytes from the start of the structure.
    pub field_offset: usize,

    pub ty: TypeName,
    pub name: &'static str,
}

#[derive(Clone)]
pub struct Class
{
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}

pub trait PluggableFields
{
    fn pluggable_fields(&self) -> Vec<Field>;
}

pub trait PluggableMethods
{
    fn pluggable_methods(&self) -> Vec<Method>;
}

/// An object that can marshall Rust values to an arbitrary value.
pub trait Marshall
{
    type Value;

    fn to_bool(value: Self::Value) -> bool;

    fn to_u8(value: Self::Value) -> u8;
    fn to_u16(value: Self::Value) -> u16;
    fn to_u32(value: Self::Value) -> u32;
    fn to_u64(value: Self::Value) -> u64;
    fn to_i8(value: Self::Value) -> i8;
    fn to_i16(value: Self::Value) -> i16;
    fn to_i32(value: Self::Value) -> i32;
    fn to_i64(value: Self::Value) -> i64;

    fn to_f32(value: Self::Value) -> f32;
    fn to_f64(value: Self::Value) -> f64;

    fn to_string(value: Self::Value) -> String;
}

/// An object that can be plugged into a scripting language.
///
/// Can be automatically derived by placing `#[pluggable]` on a struct.
///
/// When implementing `Pluggable`, it is necessary to have both a `struct` and an
/// `impl`, with both having the `#[pluggable]` attribute.
///
/// This has been split up into two other traits
///
/// * `PluggableFields`
/// * `PluggableMethods
///
/// This is because the syntax extender can not see all of the code -
/// it can only see the thing it's operating on. This is why we need
/// an attribute on the `struct` and the `impl`. We can't implement
/// a trait twice, each time overriding a specific method.
///
/// Because of this, it is necessary to mark both the `struct` and the `impl`
/// with the `pluggable` attribute. It's not the prettiest, but it does work.
pub trait Pluggable : PluggableFields + PluggableMethods
{
    fn name(&self) -> &'static str;

    fn fields(&self) -> Vec<Field> { PluggableFields::pluggable_fields(self) }
    fn methods(&self) -> Vec<Method> { PluggableMethods::pluggable_methods(self) }

    fn class(&self) -> Class {
        Class {
            name: self.name().to_owned(),
            fields: self.fields(),
            methods: self.methods(),
        }
    }
}

impl Method {
    /// Gets the marshall for a language.
    pub fn marshall(&self, lang_name: &str) -> *mut fn() {
        self.lang_marshalls.iter().find(|m| m.0 == lang_name).unwrap().1
    }
}

