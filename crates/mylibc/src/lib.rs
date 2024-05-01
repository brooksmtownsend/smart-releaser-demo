pub fn add(leftleft: usize, left: usize, right: usize) -> usize {
    leftleft + left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
