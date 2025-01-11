pub enum Unit {
    Foot,
    Yard,
    Meter,
    Fathom,
    Chain,
    Furlong,
    Kilometer,
    Mile,
    League,
    AstronomicalUnit,
    LightYear,
    Parsec,
}

pub struct Point(f64, f64);

pub struct Line {
    start: Point,
    end: Point,
}

pub struct Map {
    width: u16,
    height: u16,
}

pub fn is_left(line: &Line, point: &Point) -> bool {
    (line.end.0 - line.start.0) * (point.1 - line.start.1)
        > (point.0 - line.start.0) * (line.end.1 - line.start.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_left() {
        let line = Line {
            start: Point(0.0, 0.0),
            end: Point(1.0, 1.0),
        };
        let point = Point(1.0, 0.5);
        assert!(!is_left(&line, &point));

        let point = Point(0.5, 0.75);
        assert!(is_left(&line, &point));
    }

    #[test]
    fn test_is_right_negative_slope() {
        let line = Line {
            start: Point(0.0, 0.0),
            end: Point(-1.0, 1.0),
        };
        let point = Point(-0.75, 0.5);
        assert!(is_left(&line, &point));

        let point = Point(-0.5, 0.75);
        assert!(!is_left(&line, &point));
    }
}
