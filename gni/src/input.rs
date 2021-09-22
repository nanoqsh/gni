fn print_byte(b: u8, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for n in [b >> 4 & 0x0F, b & 0x0F] {
        write!(
            f,
            "{}",
            match n {
                0..=9 => b'0' + n,
                10..=15 => b'a' + n - 10,
                _ => unreachable!(),
            } as char
        )?
    }

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => 'l',
                Direction::Right => 'r',
                Direction::Up => 'u',
                Direction::Down => 'd',
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    CursorLeft(i8, i8),
    CursorRight(i8, i8),
    Direction(Direction),
    Left,
    Right,
    A,
    B,
    C,
    D,
    Start,
    Select,
    Quit,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct Wrap(u8);

        impl std::fmt::Display for Wrap {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                print_byte(self.0, f)
            }
        }

        write!(f, "a")?;
        write!(
            f,
            "{}",
            match *self {
                Action::CursorLeft(x, y) =>
                    return write!(f, "l{}{}", Wrap(x as u8), Wrap(y as u8)),
                Action::CursorRight(x, y) =>
                    return write!(f, "r{}{}", Wrap(x as u8), Wrap(y as u8)),
                Action::Direction(dir) => return write!(f, "d{}", dir),
                Action::Left => '>',
                Action::Right => '<',
                Action::A => 'a',
                Action::B => 'b',
                Action::C => 'c',
                Action::D => 'd',
                Action::Start => '!',
                Action::Select => '@',
                Action::Quit => 'q',
            }
        )
    }
}

pub struct Resize(pub u16, pub u16);

impl std::fmt::Display for Resize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct Wrap(u16);

        impl std::fmt::Display for Wrap {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                print_byte((self.0 >> 8 & 0xFF) as u8, f)?;
                print_byte((self.0 & 0xFF) as u8, f)
            }
        }

        write!(f, "r{}{}", Wrap(self.0), Wrap(self.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action() {
        let actual = Action::CursorLeft(0x12u8 as i8, 0xAFu8 as i8).to_string();
        let expected = "al12af";
        assert_eq!(actual, expected);

        let actual = Action::Direction(Direction::Up).to_string();
        let expected = "adu";
        assert_eq!(actual, expected);

        let actual = Action::Quit.to_string();
        let expected = "aq";
        assert_eq!(actual, expected);
    }

    #[test]
    fn resize() {
        let actual = Resize(0x1234, 0xABEF).to_string();
        let expected = "r1234abef";
        assert_eq!(actual, expected);
    }
}
