/// Enum to handle various exit status from gui functions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ExitStatus {
    COMPLETED,
    #[default]
    UNCOMPLETED,
}
