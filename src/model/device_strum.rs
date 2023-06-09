use strum_macros::{EnumString, Display};

struct Device {
    status: Status,
}

#[derive(PartialEq, Debug, EnumString, Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
enum Status {
    Reachable,
    Unreachable,
    Maintenance,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use strum::ParseError;

    use super::*;

    #[test]
    fn test_parse() {
        use Status::*;

        struct Case {
            str: &'static str,
            expected: Result<Status, ParseError>,
        }

        let cases = vec![
            Case {
                str: "REACHABLE",
                expected: Ok(Reachable),
            },
            Case {
                str: "UNREACHABLE",
                expected: Ok(Unreachable),
            },
            Case {
                str: "MAINTENANCE",
                expected: Ok(Maintenance),
            },
            Case {
                str: "x_MAINTENANCE_x",
                expected: Err(ParseError::VariantNotFound),
            },
        ];

        for case in cases {
            let actual = Status::from_str(case.str);
            assert_eq!(case.expected, actual);
        }
    }

    #[test]
    fn test_to_string() {
        use Status::*;

        struct Case {
            status: Status,
            expected: &'static str,
        }

        let cases = vec![
            Case {
                status: Reachable,
                expected: "REACHABLE",
            },
            Case {
                status: Unreachable,
                expected: "UNREACHABLE",
            },
            Case {
                status: Maintenance,
                expected: "MAINTENANCE",
            },
        ];

        for case in cases {
            let actual = case.status.to_string();
            assert_eq!(case.expected, actual);
        }
    }
}
