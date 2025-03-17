#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GoSettings {
    pub p1time: Option<i32>,
    pub p2time: Option<i32>,
    pub p1inc: Option<i32>,
    pub p2inc: Option<i32>,
    pub depth: Option<i32>,
    pub nodes: Option<u64>,
    pub movetime: Option<i32>,
    pub movestogo: Option<i32>,
    pub infinite: bool,
}

impl GoSettings {
    #[must_use]
    pub fn from_times(p1time: i32, p2time: i32, p1inc: Option<i32>, p2inc: Option<i32>) -> Self {
        Self {
            p1time: Some(p1time),
            p2time: Some(p2time),
            p1inc,
            p2inc,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn from_depth(depth: i32) -> Self {
        Self {
            depth: Some(depth),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn from_nodes(nodes: u64) -> Self {
        Self {
            nodes: Some(nodes),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn from_movetime(movetime: i32) -> Self {
        Self {
            movetime: Some(movetime),
            ..Self::default()
        }
    }
}
