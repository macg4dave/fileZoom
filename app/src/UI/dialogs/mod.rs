use crate::app::Action;

/// Map a selected button index to a runner Action, if provided.
pub fn selection_to_action(selected: usize, actions: Option<&[Action]>) -> Option<Action> {
    actions.and_then(|s| s.get(selected)).cloned()
}
