use can_types::prelude::*;

use prop::test_runner::FileFailurePersistence;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..Default::default()
    })]

    #[test]
    fn proptest_can2a_identifier_value(input in 0..u16::MAX) {
        let identifer = IdCan2A::from_bits(input);
        let max_expected = 0b11111111111;
        prop_assert!(identifer.into_bits() <= max_expected)
    }

    #[test]
    fn proptest_can2b_identifier_value(input in 0..u32::MAX) {
        let identifer = IdCan2B::from_bits(input);
        let max_expected = 0b11111111111111111111111111111;
        prop_assert!(identifer.into_bits() <= max_expected)
    }
}
