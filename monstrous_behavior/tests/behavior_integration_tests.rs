use monstrous_behavior::load_profile_spell;

#[test]
fn test_dont_pick_weakness() {
    let result = load_profile_spell(3, 2);
    assert_ne!(result, 2);
}