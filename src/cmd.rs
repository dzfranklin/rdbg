use std::fmt::Write;

use rand::Rng;

struct Token(u64);

pub(crate) struct Input {
    token: Token,
    operation: String,
    options: Vec<Opt>,
    params: Vec<Param>,
}

pub(crate) struct Opt {
    name: String,
    value: Option<String>,
}

pub(crate) struct Param(String);

pub(crate) struct Output {
    async_records: Vec<AsyncRecord>,
    stream_records: Vec<StreamRecord>,
    result: ResultRecord,
}

enum AsyncRecord {
    Exec(AsyncOutput),
    Status(AsyncOutput),
    Notify(AsyncOutput),
}

enum AsyncOutput {
    Stopped(Vec<Var>),
    Other,
}

enum StreamRecord {}

struct ResultRecord {}

struct Var {
    variable: String,
    value: VarValue,
}

impl Input {
    pub(crate) fn new(operation: String, options: Vec<Opt>, params: Vec<Param>) -> Self {
        Self {
            token: Token::generate(),
            operation,
            options,
            params,
        }
    }

    pub(crate) fn serialize(self) -> String {
        let mut options = String::new();
        for Opt { name, value } in self.options {
            if let Some(value) = value {
                write!(options, " -{name} {value}").unwrap();
            } else {
                write!(options, " -{name}").unwrap();
            }
        }

        let mut params = String::new();
        if !self.params.is_empty() {
            write!(params, " --").unwrap();
            for param in &self.params {
                write!(params, " {}", param.0).unwrap();
            }
        }

        format!(
            "{token}-{operation}{options}{params}\n",
            token = self.token.0,
            operation = self.operation,
        )
    }
}

impl Token {
    fn generate() -> Self {
        let n = rand::thread_rng().gen();
        Self(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn input_fixture(operation: impl Into<String>, options: Vec<Opt>, params: Vec<Param>) -> Input {
        Input {
            token: Token(0),
            operation: operation.into(),
            options,
            params,
        }
    }

    fn opt(name: impl Into<String>) -> Opt {
        Opt {
            name: name.into(),
            value: None,
        }
    }

    fn opt_v(name: impl Into<String>, value: impl Into<String>) -> Opt {
        Opt {
            name: name.into(),
            value: Some(value.into()),
        }
    }

    fn param(name: impl Into<String>) -> Param {
        Param(name.into())
    }

    #[test]
    fn serialize_input_no_params() {
        let subject = input_fixture("op", vec![opt("foo"), opt_v("foo2", "bar")], vec![]);
        assert_eq!(subject.serialize(), "0-op -foo -foo2 bar\n")
    }

    #[test]
    fn serialize_input_params() {
        let subject = input_fixture(
            "op",
            vec![opt("foo"), opt_v("foo2", "bar")],
            vec![param("p1"), param("p2")],
        );
        assert_eq!(subject.serialize(), "0-op -foo -foo2 bar -- p1 p2\n")
    }
}
