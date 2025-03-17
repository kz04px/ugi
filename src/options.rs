use std::fmt::{self, Display};

#[derive(Default)]
pub struct Check {
    pub name: String,
    pub value: bool,
}

pub struct Spin<T: PartialOrd + PartialEq + Display> {
    pub name: String,
    pub min: T,
    pub max: T,
    pub value: T,
}

pub struct Combo {
    pub name: String,
    pub value: String,
    pub options: Vec<String>,
}

pub struct Button {
    pub name: String,
}

pub struct Stringy {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type check default {}",
            self.name, self.value
        )?;
        Ok(())
    }
}

impl<T: PartialOrd + PartialEq + Display> fmt::Display for Spin<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type spin default {} min {} max {}",
            self.name, self.value, self.min, self.max
        )?;
        Ok(())
    }
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type combo default {}",
            self.name, self.value
        )?;
        for word in &self.options {
            write!(f, " var {}", word)?;
        }
        Ok(())
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "option name {} type button", self.name)?;
        Ok(())
    }
}

impl fmt::Display for Stringy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type string default {}",
            self.name, self.value
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_displayed() {
        let option = Check {
            name: "Debug".to_owned(),
            value: false,
        };
        assert_eq!(
            option.to_string(),
            "option name Debug type check default false"
        );
    }

    #[test]
    fn spin_is_displayed() {
        let option = Spin {
            name: "Hash".to_owned(),
            min: 1,
            max: 1000,
            value: 16,
        };
        assert_eq!(
            option.to_string(),
            "option name Hash type spin default 16 min 1 max 1000"
        );
    }

    #[test]
    fn combo_is_displayed() {
        let option = Combo {
            name: "Style".to_owned(),
            value: "Normal".to_owned(),
            options: vec!["Normal".to_owned(), "Risky".to_owned()],
        };
        assert_eq!(
            option.to_string(),
            "option name Style type combo default Normal var Normal var Risky"
        );
    }

    #[test]
    fn button_is_displayed() {
        let option = Button {
            name: "Clear".to_owned(),
        };
        assert_eq!(option.to_string(), "option name Clear type button");
    }

    #[test]
    fn stringy_is_displayed() {
        let option = Stringy {
            name: "EvalFile".to_owned(),
            value: "nn.bin".to_owned(),
        };
        assert_eq!(
            option.to_string(),
            "option name EvalFile type string default nn.bin"
        );
    }
}
