#![allow(dead_code)]

use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify::Tsify;

#[test]
fn test_unit() {
    #[derive(Tsify)]
    struct Unit;

    assert_eq!(Unit::DECL, "export type Unit = null;");
}

#[test]
fn test_named_fields() {
    #[derive(Tsify)]
    struct A {
        a: (u8, u8),
        b: String,
    }

    assert_eq!(
        A::DECL,
        indoc! {"
            export interface A {
                a: [number, number];
                b: string;
            }"
        }
    );
}

#[test]
fn test_newtype_struct() {
    #[derive(Tsify)]
    struct Newtype(i32);

    assert_eq!(Newtype::DECL, "export type Newtype = number;");
}

#[test]
fn test_tuple_struct() {
    #[derive(Tsify)]
    struct Tuple(i32, String);
    #[derive(Tsify)]
    struct EmptyTuple();

    assert_eq!(Tuple::DECL, "export type Tuple = [number, string];");
    assert_eq!(EmptyTuple::DECL, "export type EmptyTuple = [];");
}

#[test]
fn test_nested_struct() {
    #[derive(Tsify)]
    struct A {
        x: f64,
    }

    #[derive(Tsify)]
    struct B {
        a: A,
    }

    assert_eq!(
        B::DECL,
        indoc! {"
            export interface B {
                a: A;
            }"
        }
    );
}

#[test]
fn test_struct_with_borrowed_fields() {
    use std::borrow::Cow;

    #[derive(Tsify)]
    struct Borrow<'a> {
        raw: &'a str,
        cow: Cow<'a, str>,
    }

    assert_eq!(
        Borrow::DECL,
        indoc! {"
            export interface Borrow {
                raw: string;
                cow: string;
            }"
        }
    );
}

#[test]
fn test_tagged_struct() {
    #[derive(Tsify)]
    #[serde(tag = "type")]
    struct TaggedStruct {
        x: i32,
        y: i32,
    }

    assert_eq!(
        TaggedStruct::DECL,
        indoc! {r#"
            export interface TaggedStruct {
                type: "TaggedStruct";
                x: number;
                y: number;
            }"#
        }
    );
}
