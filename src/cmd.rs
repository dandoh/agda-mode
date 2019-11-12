use crate::base::{ComputeMode, InteractionPoint, Rewrite};

/// How much highlighting should be sent to the user interface?
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HighlightingLevel {
    None,
    NonInteractive,
    /// This includes both non-interactive highlighting and
    /// interactive highlighting of the expression that is currently
    /// being type-checked.
    Interactive,
}

/// How should highlighting be sent to the user interface?
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HighlightingMethod {
    /// Via stdout.
    Direct,
    /// Both via files and via stdout.
    Indirect,
}

#[derive(Debug, Clone)]
pub struct IOTCM {
    level: HighlightingLevel,
    method: HighlightingMethod,
    command: Cmd,
}

impl IOTCM {
    pub fn new(level: HighlightingLevel, method: HighlightingMethod, command: Cmd) -> Self {
        Self {
            level,
            method,
            command,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoalInput {
    id: InteractionPoint,
    // TODO: range
    code: String,
}

#[derive(Debug, Clone)]
pub enum Cmd {
    Load {
        path: String,
        flags: Vec<String>,
    },
    Compile {
        backend: String,
        path: String,
        flags: Vec<String>,
    },
    Constraints,
    Metas,
    ShowModuleContentsToplevel {
        rewrite: Rewrite,
        search: String,
    },
    SolveAll {
        rewrite: Rewrite,
    },
    SolveOne {
        rewrite: Rewrite,
        input: GoalInput,
    },
    AutoOne {
        input: GoalInput,
    },
    AutoAll,
    InferToplevel {
        rewrite: Rewrite,
        code: String,
    },
    ComputeToplevel {
        rewrite: ComputeMode,
        code: String,
    },
}
