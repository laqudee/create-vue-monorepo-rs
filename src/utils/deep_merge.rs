use serde_json::{json, Value};

fn merge_array_with_dedupe(a: &Value, b: &Value) -> Value {
    let mut result = Vec::new();

    if let Value::Array(a) = a {
        result.extend(a.iter().cloned());
    }

    if let Value::Array(b) = b {
        result.extend(b.iter().cloned());
    }

    result.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    result.dedup();

    Value::Array(result)
}

pub fn merge(target: &Value, obj: &Value) -> Value {
    let mut target = target.clone();
    for key in obj.as_object().unwrap().keys() {
        let old_val = {
            if let Some(val) = target.get(key) {
                val
            } else {
                &json!(null)
            }
        };
        let new_val = obj.get(key).unwrap();
        // *old_val = new_val.clone();

        if old_val.is_array() && new_val.is_array() {
            *target.get_mut(key).unwrap() = merge_array_with_dedupe(old_val, new_val);
        } else if old_val.is_object() && new_val.is_object() {
            *target.get_mut(key).unwrap() = merge(&old_val, &new_val);
        } else {
            // println!("target key: {:?} - {:?}", key, target.get(key));
            // println!("new_val key: {:?} - {:?}", key, obj.get(key));

            if let Some(val) = target.get_mut(key) {
                *val = new_val.clone()
            }
        }
    }
    // println!("finally: target: {:?}", target);
    target.into()
}
