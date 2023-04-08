use serde_json::Value;

pub fn is_object(val: Value) -> bool {
    val.is_object()
}
