use quant_core::models::maths;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = maths::add(5.0, 10.0);
        assert_eq!(result, 15.0);
        println!("test result add {}", result);
    }


    
}