use serde_json::{Map, Value};

// TODO 排序也有问题
pub fn sort(package_json: &mut Value) -> Value {
    let mut sorted = Map::new();

    let dep_types = vec![
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ];

    for dep_type in dep_types {
        if let Some(deps) = package_json.get_mut(dep_type) {
            let mut sorted_deps = Map::new();

            if let Some(deps_obj) = deps.as_object_mut() {
                let mut dep_names: Vec<&String> = deps_obj.keys().collect();
                dep_names.sort();

                for name in dep_names {
                    if let Some(dep) = deps_obj.get(name) {
                        sorted_deps.insert(name.clone(), dep.clone());
                    }
                }
            }

            sorted.insert(dep_type.to_string(), Value::Object(sorted_deps));
        }
    }

    let mut result = package_json.clone();
    for (key, value) in sorted {
        if let Some(obj) = value.as_object() {
            if let Some(result_obj) = result.as_object_mut() {
                result_obj.insert(key, Value::Object(obj.clone()));
            }
        }
    }

    result
}
