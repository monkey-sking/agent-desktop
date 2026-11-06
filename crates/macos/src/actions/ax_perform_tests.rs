use super::imp::activation_plan;

fn actions(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

#[test]
fn an_advertised_press_is_the_only_action_attempted() {
    assert_eq!(
        activation_plan(Some(&actions(&["AXPress", "AXShowMenu"]))),
        vec!["AXPress"]
    );
}

#[test]
fn open_only_items_are_activated_with_open() {
    assert_eq!(activation_plan(Some(&actions(&["AXOpen"]))), vec!["AXOpen"]);
    assert_eq!(
        activation_plan(Some(&actions(&["AXShowMenu", "AXOpen"]))),
        vec!["AXOpen"]
    );
}

#[test]
fn items_with_neither_action_are_never_pressed() {
    assert!(activation_plan(Some(&actions(&["AXShowMenu"]))).is_empty());
    assert!(activation_plan(Some(&[])).is_empty());
}

#[test]
fn an_unread_action_list_keeps_the_conventional_order() {
    assert_eq!(activation_plan(None), vec!["AXPress", "AXOpen"]);
}
