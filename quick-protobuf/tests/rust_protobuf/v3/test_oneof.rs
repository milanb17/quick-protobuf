// generated
include!("../common/test_oneof.rs");

#[test]
fn test_types_v3() {
    t(|o| {
        let mut msg = MessageForOneof::default();
        msg.f = 22;
        o.one = mod_TestOneof::OneOfone::message_field(msg);
    })
}
