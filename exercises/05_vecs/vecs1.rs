fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;
    // let v = a.into();
    let v = Vec::from(a);
    (a, v)
}

// array -> vec
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// let v = Vec::from(a); // Using Vec::from()
// // Or:
// let v: Vec<_> = a.into(); // Using Into::into()

// let a: [i32; 5] = [1, 2, 3, 4, 5];
// let v = a.to_vec(); // Converts the array to a slice, then to a Vec

// let a: [i32; 5] = [1, 2, 3, 4, 5];
// let v: Vec<i32> = a.iter().cloned().collect();

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
