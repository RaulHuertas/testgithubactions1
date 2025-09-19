

pub fn multiply(a: i32, b : i32)->i32{
    a*b
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1(){
        let result = multiply(8,9);
        assert_eq!(result,72);
    }
}


