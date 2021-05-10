use super::*;
use crate::mock::{new_test_ext, Event, GameTemplate, Origin, System, Test};
use crate::{mock::*, Error};
use frame_support::{
    assert_noop, assert_ok,
    traits::{OnFinalize, OnInitialize},
};

// 【Scenario】test create template success
#[test]
fn create_template() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange

        // 【When】Act
        assert_ok!(GameTemplate::create_template(Origin::signed(1), 100));

        // 【Then】Assert
        // check template created
        let templates = GameTemplate::get_templates();
        assert_eq!(templates.len(), 1);
    });
}

// 【Scenario】test create template faild
#[test]
fn create_template_failded_when_permission_denied() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange

        // 【When】Act
        // 【Then】Assert
        assert_noop!(
            GameTemplate::create_template(Origin::signed(2), 1000),
            Error::<Test>::PermissionDenied
        );
        // check template list
        let templates = GameTemplate::get_templates();
        assert_eq!(templates.len(), 0);
    });
}
