
use array2d::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Move { dir: Direction },
    Enter,
    Cancel,
    OpenExitWin, OpenItemMenu, OpenEquipWin, PickUpItem,
    TextInput { text: String },
    TextDelete,
    /// Special command to notify that the child window is closed
    ChildWindowClosed,
}
