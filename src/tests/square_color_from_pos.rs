#[cfg(test)]
mod tests {
    use crate::chess_game::should_be_light_square;

    #[test]
    fn test_square_is_white_1() {
        let pos = 3;

        let is_white = should_be_light_square(pos);

        assert_eq!(true, is_white)
    }

    #[test]
    fn test_square_is_black_1() {
        let pos = 4;

        let is_white = should_be_light_square(pos);

        assert_eq!(false, is_white)
    }

    #[test]
    fn test_square_is_white_2() {
        let pos = 37;

        let is_white = should_be_light_square(pos);

        assert_eq!(true, is_white)
    }

    #[test]
    fn test_square_is_black_2() {
        let pos = 63;

        let is_white = should_be_light_square(pos);

        assert_eq!(false, is_white)
    }
}
