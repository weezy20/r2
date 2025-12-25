use ron::value::Value;

fn ron_to_json(ron_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = ron_str.parse()?;
    let json_string = serde_json::to_string_pretty(&ron_value)?;
    Ok(json_string)
}

fn json_to_ron(json_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = serde_json::from_str(json_str)?;
    let ron_string = ron::ser::to_string_pretty(&ron_value, ron::ser::PrettyConfig::default())?;
    Ok(ron_string)
}

fn ron_to_yaml(ron_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = ron_str.parse()?;
    let yaml_string = serde_yaml_bw::to_string(&ron_value)?;
    Ok(yaml_string)
}

fn yaml_to_ron(yaml_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = serde_yaml_bw::from_str(yaml_str)?;
    let ron_string = ron::ser::to_string_pretty(&ron_value, ron::ser::PrettyConfig::default())?;
    Ok(ron_string)
}

fn ron_to_toml(ron_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = ron_str.parse()?;
    let toml_string = toml::to_string_pretty(&ron_value)?;
    Ok(toml_string)
}

fn toml_to_ron(toml_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ron_value: Value = toml::from_str(toml_str)?;
    let ron_string = ron::ser::to_string_pretty(&ron_value, ron::ser::PrettyConfig::default())?;
    Ok(ron_string)
}

fn round_trip_ron_json(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = ron_to_json(original)?;
    let ron_back = json_to_ron(&json)?;
    let ron_value: Value = original.parse()?;
    let ron_back_value: Value = ron_back.parse()?;
    assert_eq!(ron_value, ron_back_value);
    Ok(())
}

fn round_trip_ron_yaml(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yaml = ron_to_yaml(original)?;
    let ron_back = yaml_to_ron(&yaml)?;
    let ron_value: Value = original.parse()?;
    let ron_back_value: Value = ron_back.parse()?;
    assert_eq!(ron_value, ron_back_value);
    Ok(())
}

fn round_trip_ron_toml(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml = ron_to_toml(original)?;
    let ron_back = toml_to_ron(&toml)?;
    let ron_value: Value = original.parse()?;
    let ron_back_value: Value = ron_back.parse()?;
    assert_eq!(ron_value, ron_back_value);
    Ok(())
}

fn round_trip_json_ron(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ron = json_to_ron(original)?;
    let json_back = ron_to_json(&ron)?;
    let json_value: Value = serde_json::from_str(original)?;
    let json_back_value: Value = serde_json::from_str(&json_back)?;
    assert_eq!(json_value, json_back_value);
    Ok(())
}

fn round_trip_yaml_ron(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ron = yaml_to_ron(original)?;
    let yaml_back = ron_to_yaml(&ron)?;
    let yaml_value: Value = serde_yaml_bw::from_str(original)?;
    let yaml_back_value: Value = serde_yaml_bw::from_str(&yaml_back)?;
    assert_eq!(yaml_value, yaml_back_value);
    Ok(())
}

fn round_trip_toml_ron(original: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ron = toml_to_ron(original)?;
    let toml_back = ron_to_toml(&ron)?;
    let toml_value: Value = toml::from_str(original)?;
    let toml_back_value: Value = toml::from_str(&toml_back)?;
    assert_eq!(toml_value, toml_back_value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ron_to_json() {
        let ron = r#"GameConfig(
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,
)"#;
        let result = ron_to_json(ron);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_to_ron() {
        let json = r#"{
    "hello": "world",
    "is_this_a_test": true
}"#;
        let result = json_to_ron(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ron_to_yaml() {
        let ron = r#"(
    window_size: (800, 600),
    window_title: "PAC-MAN",
)"#;
        let result = ron_to_yaml(ron);
        assert!(result.is_ok());
    }

    #[test]
    fn test_yaml_to_ron() {
        let yaml = r#"hello: world
is_this_a_test: true"#;
        let result = yaml_to_ron(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ron_to_toml() {
        let ron = r#"(
    window_size: (800, 600),
    window_title: "PAC-MAN",
)"#;
        let result = ron_to_toml(ron);
        assert!(result.is_ok());
    }

    #[test]
    fn test_toml_to_ron() {
        let toml = r#"hello = "world"
is_this_a_test = true"#;
        let result = toml_to_ron(toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_round_trip_ron_json_simple() {
        let ron = r#"(
    name: "test",
    value: 42,
    active: true,
)"#;
        assert!(round_trip_ron_json(ron).is_ok());
    }

    #[test]
    fn test_round_trip_ron_json_complex() {
        let ron = r#"GameConfig(
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,
    mouse_sensitivity: 1.4,
    difficulty_options: (
        start_difficulty: "Easy",
        adaptive: false,
    ),
)"#;
        assert!(round_trip_ron_json(ron).is_ok());
    }

    #[test]
    fn test_round_trip_ron_yaml_simple() {
        let ron = r#"(
    name: "test",
    value: 42,
    active: true,
)"#;
        assert!(round_trip_ron_yaml(ron).is_ok());
    }

    #[test]
    fn test_round_trip_ron_yaml_complex() {
        let ron = r#"(
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,
    mouse_sensitivity: 1.4,
)"#;
        assert!(round_trip_ron_yaml(ron).is_ok());
    }

    #[test]
    fn test_round_trip_ron_toml_simple() {
        let ron = r#"(
    name: "test",
    value: 42,
    active: true,
)"#;
        assert!(round_trip_ron_toml(ron).is_ok());
    }

    #[test]
    fn test_round_trip_ron_toml_complex() {
        let ron = r#"(
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,
    mouse_sensitivity: 1.4,
)"#;
        assert!(round_trip_ron_toml(ron).is_ok());
    }

    #[test]
    fn test_round_trip_json_ron_simple() {
        let json = r#"{
    "name": "test",
    "value": 42,
    "active": true
}"#;
        assert!(round_trip_json_ron(json).is_ok());
    }

    #[test]
    fn test_round_trip_json_ron_complex() {
        let json = r#"{
    "window_size": [800, 600],
    "window_title": "PAC-MAN",
    "fullscreen": false,
    "mouse_sensitivity": 1.4
}"#;
        assert!(round_trip_json_ron(json).is_ok());
    }

    #[test]
    fn test_round_trip_yaml_ron_simple() {
        let yaml = r#"name: test
value: 42
active: true"#;
        assert!(round_trip_yaml_ron(yaml).is_ok());
    }

    #[test]
    fn test_round_trip_yaml_ron_complex() {
        let yaml = r#"window_size:
- 800
- 600
window_title: PAC-MAN
fullscreen: false
mouse_sensitivity: 1.4"#;
        assert!(round_trip_yaml_ron(yaml).is_ok());
    }

    #[test]
    fn test_round_trip_toml_ron_simple() {
        let toml = r#"name = "test"
value = 42
active = true"#;
        assert!(round_trip_toml_ron(toml).is_ok());
    }

    #[test]
    fn test_round_trip_toml_ron_complex() {
        let toml = r#"window_size = [800, 600]
window_title = "PAC-MAN"
fullscreen = false
mouse_sensitivity = 1.4"#;
        assert!(round_trip_toml_ron(toml).is_ok());
    }

    #[test]
    fn test_nested_structures_ron_json() {
        let ron = r#"(
    user: (
        name: "Alice",
        age: 30,
        address: (
            city: "Boston",
            country: "USA",
        ),
    ),
    tags: ["work", "personal"],
)"#;
        assert!(round_trip_ron_json(ron).is_ok());
    }

    #[test]
    fn test_nested_structures_ron_yaml() {
        let ron = r#"(
    user: (
        name: "Alice",
        age: 30,
        address: (
            city: "Boston",
            country: "USA",
        ),
    ),
    tags: ["work", "personal"],
)"#;
        assert!(round_trip_ron_yaml(ron).is_ok());
    }

    #[test]
    fn test_nested_structures_ron_toml() {
        let ron = r#"(
    user: (
        name: "Alice",
        age: 30,
        address: (
            city: "Boston",
            country: "USA",
        ),
    ),
    tags: ["work", "personal"],
)"#;
        assert!(round_trip_ron_toml(ron).is_ok());
    }

    #[test]
    fn test_empty_map_and_array_ron_json() {
        let ron = r#"(
    items: [],
    config: {},
)"#;
        assert!(round_trip_ron_json(ron).is_ok());
    }

    #[test]
    fn test_empty_map_and_array_ron_yaml() {
        let ron = r#"(
    items: [],
    config: {},
)"#;
        assert!(round_trip_ron_yaml(ron).is_ok());
    }

    #[test]
    fn test_empty_map_and_array_ron_toml() {
        let ron = r#"(
    items: [],
    config: {},
)"#;
        assert!(round_trip_ron_toml(ron).is_ok());
    }

    #[test]
    fn test_numbers_and_special_values_ron_json() {
        let ron = r#"(
    integer: 42,
    negative: -10,
    float: 3.14,
    zero: 0,
)"#;
        assert!(round_trip_ron_json(ron).is_ok());
    }

    #[test]
    fn test_numbers_and_special_values_ron_yaml() {
        let ron = r#"(
    integer: 42,
    negative: -10,
    float: 3.14,
    zero: 0,
)"#;
        assert!(round_trip_ron_yaml(ron).is_ok());
    }

    #[test]
    fn test_numbers_and_special_values_ron_toml() {
        let ron = r#"(
    integer: 42,
    negative: -10,
    float: 3.14,
    zero: 0,
)"#;
        assert!(round_trip_ron_toml(ron).is_ok());
    }
}
