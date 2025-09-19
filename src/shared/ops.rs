mod shared{

mod ops{    

pub fn multiply(a: int32, b : int32){
    return a+b;
}
}


}//end of mod shared

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1(){
        let result = ops::multiply(8,9);
        assert_eq!(result,27)
    }
}


