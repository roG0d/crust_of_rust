#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };

    // Syntax needed to state that can be 1 or more expressions followed by , 
    ($($element: expr),+) => {{
        let mut vs = Vec::new();
        // Syntax needed to state instruction repetition over an expression
        $(vs.push($element);)+
        vs
    }};

    // Match to add the same $element $count times
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element;
        for _ in 0..$count {
            vs.push(x.clone());
        }
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

#[test]
fn empty_vec(){
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

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

 
fn trailing(){
    let x: Vec<u32> = avec![1,2,3,4,
    5,6,7,8,9,10];
    assert!(!x.is_empty());
    assert_eq!(x.len(),2);
    assert_eq!(x[0], 42);
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