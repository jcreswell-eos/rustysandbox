pub fn load_profile_spell(left: u64, right: u64) -> u64 {
    // todo: make some kinda AI behavior profile struct that hosts tags to define behavior patterns and ability mappings
    if left >= right {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests that an input AI with profile tags for 'magic' under roles and 'aggressive' under tactics will select the spell from its profile with the highest damage output assuming nothing is known about target profile's weaknesses/strengths.
     */
    #[test]
    fn test_load_profile_agressive_magic() {
        // todo: create a dummy profile with 'magic' and 'aggressive' tags and a couple spells with varying damage output levels.
        let result = load_profile_spell(2, 3);
        let expected_val = 3;
        assert_eq!(result, expected_val, "Highest value was not {:?}, instead it came back as {:?}", expected_val, result);
    }

    #[test]
    #[should_panic(expected = "faulty")]
    fn test_faulted() {
        panic!("oh snap we faulty");
    }
}
