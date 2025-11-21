// ReqIF data model - Core data structures mirroring ReqIF 1.2 schema

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root ReqIF document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqIF {
    pub header: ReqIFHeader,
    pub core_content: CoreContent,
    #[serde(default)]
    pub tool_extensions: Vec<ToolExtension>,
}

/// ReqIF header with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqIFHeader {
    pub identifier: String,
    pub creation_time: String,
    pub source_tool_id: String,
    pub title: Option<String>,
    pub comment: Option<String>,
}

/// Core content containing all specifications and requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreContent {
    #[serde(default)]
    pub spec_objects: Vec<SpecObject>,
    #[serde(default)]
    pub spec_relations: Vec<SpecRelation>,
    #[serde(default)]
    pub specifications: Vec<Specification>,
    #[serde(default)]
    pub spec_types: Vec<SpecType>,
    #[serde(default)]
    pub datatype_definitions: Vec<DatatypeDefinition>,
}

/// Individual requirement or specification object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecObject {
    pub identifier: String,
    pub spec_type: String, // Reference to SpecType
    pub last_change: Option<String>,
    #[serde(default)]
    pub values: Vec<AttributeValue>,
    /// Preserve unknown XML elements for round-trip
    #[serde(default)]
    pub extra_attrs: HashMap<String, String>,
}

/// Link between requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecRelation {
    pub identifier: String,
    pub spec_type: String,
    pub source: String, // SpecObject ID
    pub target: String, // SpecObject ID
    pub last_change: Option<String>,
    #[serde(default)]
    pub values: Vec<AttributeValue>,
}

/// Hierarchical specification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub identifier: String,
    pub spec_type: String,
    pub last_change: Option<String>,
    #[serde(default)]
    pub values: Vec<AttributeValue>,
    #[serde(default)]
    pub children: Vec<SpecHierarchy>,
}

/// Hierarchy node referencing a SpecObject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecHierarchy {
    pub identifier: String,
    pub object: String, // SpecObject ID reference
    pub last_change: Option<String>,
    #[serde(default)]
    pub children: Vec<SpecHierarchy>,
}

/// Type definition for SpecObjects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecType {
    pub identifier: String,
    pub long_name: Option<String>,
    pub description: Option<String>,
    pub last_change: Option<String>,
    #[serde(default)]
    pub spec_attributes: Vec<AttributeDefinition>,
}

/// Attribute definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDefinition {
    pub identifier: String,
    pub long_name: Option<String>,
    pub datatype_ref: String,
    pub last_change: Option<String>,
}

/// Datatype definition (Boolean, Integer, Real, String, Enumeration, XHTML)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DatatypeDefinition {
    Boolean {
        identifier: String,
        long_name: Option<String>,
    },
    Integer {
        identifier: String,
        long_name: Option<String>,
        min: Option<i64>,
        max: Option<i64>,
    },
    Real {
        identifier: String,
        long_name: Option<String>,
        min: Option<f64>,
        max: Option<f64>,
        accuracy: Option<u32>,
    },
    String {
        identifier: String,
        long_name: Option<String>,
        max_length: Option<u32>,
    },
    Enumeration {
        identifier: String,
        long_name: Option<String>,
        values: Vec<EnumValue>,
    },
    XHTML {
        identifier: String,
        long_name: Option<String>,
    },
}

/// Enumeration value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub identifier: String,
    pub long_name: Option<String>,
    pub properties: Option<String>,
}

/// Attribute value (typed)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttributeValue {
    Boolean {
        definition: String,
        value: bool,
    },
    Integer {
        definition: String,
        value: i64,
    },
    Real {
        definition: String,
        value: f64,
    },
    String {
        definition: String,
        value: String,
    },
    Enumeration {
        definition: String,
        value: String, // EnumValue ID reference
    },
    XHTML {
        definition: String,
        value: String, // XHTML content as string
    },
}

/// Tool-specific extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExtension {
    pub identifier: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_object_creation() {
        let spec_obj = SpecObject {
            identifier: "REQ-001".to_string(),
            spec_type: "requirement-type".to_string(),
            last_change: None,
            values: vec![],
            extra_attrs: HashMap::new(),
        };
        assert_eq!(spec_obj.identifier, "REQ-001");
    }

    #[test]
    fn test_attribute_value_serialization() {
        let attr = AttributeValue::String {
            definition: "attr-def-1".to_string(),
            value: "Test requirement".to_string(),
        };
        let json = serde_json::to_string(&attr).unwrap();
        assert!(json.contains("Test requirement"));
    }
}
