#[macro_export]
macro_rules! avec {
     
    // Syntax needed to state that can be 1 or more expressions followed by ", " 
    ($($element: expr),+) => {{


        // Check that coun is const (so it occurs at compile time)
        const _:usize = $crate::avec![@COUNT; $($element),*];


        // We need a macro to create the count value nedeed for Vec::with_capacity()
        // let count = [$(element),*].len(); Not ok beacuse we reevaluate every expression multiple times
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        // Syntax needed to state instruction repetition over an expression
        $(vs.push($element);)+
        vs
    }};

    // Match to add the same $element $count times
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        //More efficient than a loop cause it gets rid of the pointer increment in the vec and adds elements directly in it
        vs.resize($count, $element);
        /*
        let x = $element;
        for _ in 0..count {
            vs.push(x.clone());
        }*/
        vs 

    }};

    // Match to add the same $element $count times (Being $element non literal, more complex expression)
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        for _ in 0..$count {
            vs.push($element);
        }
        vs
    }};
    

    // Internal macro feature to accomplish the counting of variable numbers of parameters in with_capacity()
    (@COUNT; $($element:expr),*) => {

        // We use @SUBST to not take the element yielded by @COUNT, as rust need an expression with a variable to look at we need @SUBST
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*])
        // <[()]>::len Here we are calling the method .len from a slice, we can use this because as a &str (notice the & inmediatly after) has the as_ref_slice() 
        // we can use every method that reside on Slice
    };

    // When @SUBST is expanded there is nothing more but the only computation of the macro
    (@SUBST; $_element:expr) => { ( ) };

}

// Extra work: Sample implementation for HashMap
use std::{collections::HashMap};

#[macro_export]
macro_rules! ahashmap {
    ($($key: expr => $value: expr),+) => {{

        let mut hm  = HashMap::new();
        $(hm.insert($key, $value);)+
        hm
    }};

}

// Simple trait
trait MaxValue {
    fn max_value() -> Self;
}
// Generic trait implementation for every type using macro rules
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t{
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };  
}

max_impl!(i32);
max_impl!(u32);
max_impl!(i64);
max_impl!(u64);

// We can use cargo expand --lib --tests to see the expansion of every macro
#[test]
fn single(){
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(),1);
    assert_eq!(x[0], 42);
}

#[test] 
fn double(){
    let x: Vec<u32> = avec![42,43];
    assert!(!x.is_empty());
    assert_eq!(x.len(),2);
    assert_eq!(x[0], 42);
}

#[test]
fn trailing(){
    let _: Vec<&'static str> = avec![
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg",
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg",
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg",
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg",
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg",
        "kjasdjakisdfhkjasfhdgkjafghkjahfgjkashfkjghadkjfghakjdfg"
    ];
}


#[test]
fn clone_2(){
    let x: Vec<u32> = avec![3; 3];
    assert_eq!(x[0], 3);
    assert_eq!(x[1], 3);
    assert_eq!(x[2], 3);
}

#[test]
fn clone_2_non_literal(){
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 3];
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}

// A trick to make failing test
/// ```compile_fail
/// let x:Vec <u32> = vecmac::avec![42;"foo"];
/// ```
#[allow(dead_code)]
pub struct CompileFaileTest;


#[test]
fn single_hashmap(){
    let x: HashMap<&str,u32> = ahashmap!{"one" => 1};
    assert!(!x.is_empty());
    assert_eq!(x.len(),1);
    assert_eq!(x.get("one"), Some(&1));
}

#[test] 
fn double_hashmap(){
    let x: HashMap<&str,u32> = ahashmap!{
                                        "one" => 1,
                                        "two" => 2
                                        };
    assert!(!x.is_empty());
    assert_eq!(x.len(),2);
    assert_eq!(x.get("two"), Some(&2));
}