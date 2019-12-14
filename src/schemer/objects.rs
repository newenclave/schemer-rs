#![allow(unused)]

struct StringType {}
struct IntegerType {}
struct FloatingType {}
struct BooleanType {}
struct ObjectType {}

struct IntegerTypeInfo {}
struct FloatingTypeInfo {}
struct BooleanTypeInfo {}
struct ObjectTypeInfo {}


struct TypeInfo<BaseType: std::clone::Clone> {
    prototype: BaseType,
}

enum Element {
    None,
    Str(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
}

