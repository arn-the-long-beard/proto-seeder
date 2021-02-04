pub const _GUARD_TEMPLATE: &str = r###"
pub fn GUARD_NAME(PAYLOAD) -> Option<bool>
{
if 1+1 ==2 {
 Err("Need to implement nice conditions");
}

None
}
"###;
