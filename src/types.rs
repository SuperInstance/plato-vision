use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Text representation of what a camera sees.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDescription {
    pub summary: String,
    pub detail: String,
    pub objects: Vec<String>,
    pub motion_detected: bool,
    pub timestamp: u64,
}

impl SceneDescription {
    pub fn empty() -> Self {
        Self {
            summary: "empty scene".into(),
            detail: "No objects or motion detected in the frame.".into(),
            objects: vec![],
            motion_detected: false,
            timestamp: now_ms(),
        }
    }
}

/// A detected object in a scene.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
    pub position: BoundingBox,
    pub attributes: Vec<String>,
}

/// Axis-aligned bounding box.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// A change observed between two frames.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    Appeared,
    Disappeared,
    Moved,
    Changed,
}

/// Description of a scene change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneChange {
    pub change_type: ChangeType,
    pub description: String,
    pub object_label: String,
}

/// Severity level for alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// An alert generated from scene analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneAlert {
    pub severity: Severity,
    pub description: String,
    pub timestamp: u64,
}

/// Camera configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub resolution: (u32, u32),
    pub fps: u32,
    pub region_of_interest: Option<BoundingBox>,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            fps: 30,
            region_of_interest: None,
        }
    }
}

/// A region of the frame that should be masked for privacy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyMask {
    pub region: BoundingBox,
    pub label: String,
}

/// Raw frame data (simplified – in production this would be image bytes).
#[derive(Debug, Clone)]
pub struct FrameData {
    pub objects: Vec<DetectedObject>,
    pub motion_detected: bool,
    pub timestamp: u64,
}

impl FrameData {
    pub fn empty() -> Self {
        Self {
            objects: vec![],
            motion_detected: false,
            timestamp: now_ms(),
        }
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
