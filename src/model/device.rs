use std::{fmt::Display, str::FromStr};

struct Device {
    status: Status,
}

#[derive(PartialEq, Debug)]
enum Status {
    Reachable,
    Unreachable,
    Maintenance,
}

// Manual implementations to illustrate the use of standard traits.

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REACHABLE" => Ok(Status::Reachable),
            "UNREACHABLE" => Ok(Status::Unreachable),
            "MAINTENANCE" => Ok(Status::Maintenance),
            _ => Err(()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Reachable => "REACHABLE",
            Status::Unreachable => "UNREACHABLE",
            Status::Maintenance => "MAINTENANCE",
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        use Status::*;

        struct Case {
            str: &'static str,
            expected: Result<Status, ()>,
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
                expected: Err(()),
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
