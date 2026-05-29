use crate::types::*;

use std::collections::HashMap;

/// The main vision engine.
pub struct Vision {
    config: CameraConfig,
    /// Accumulated baseline: object label → last known count / presence.
    baseline: HashMap<String, usize>,
}

impl Vision {
    pub fn new(config: CameraConfig) -> Self {
        Self {
            config,
            baseline: HashMap::new(),
        }
    }

    /// Produce a text description of a single frame.
    pub fn describe_frame(&self, frame: &FrameData) -> SceneDescription {
        if frame.objects.is_empty() && !frame.motion_detected {
            return SceneDescription::empty();
        }

        let object_names: Vec<String> = frame.objects.iter().map(|o| o.label.clone()).collect();
        let label_counts = count_labels(&object_names);

        let mut parts: Vec<String> = label_counts
            .iter()
            .map(|(label, count)| {
                if *count > 1 {
                    format!("{count} {label}s")
                } else {
                    label.clone()
                }
            })
            .collect();
        parts.sort();

        let summary = if parts.is_empty() {
            "Scene with motion but no identifiable objects.".into()
        } else {
            format!("Scene containing {}.", parts.join(", "))
        };

        let detail = if frame.objects.is_empty() {
            "Motion detected but no objects could be identified.".into()
        } else {
            frame
                .objects
                .iter()
                .map(|o| {
                    format!(
                        "{} (confidence {:.0}%) at ({:.0},{:.0})",
                        o.label,
                        o.confidence * 100.0,
                        o.position.x,
                        o.position.y,
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        };

        SceneDescription {
            summary,
            detail,
            objects: object_names,
            motion_detected: frame.motion_detected,
            timestamp: frame.timestamp,
        }
    }

    /// Detect objects in a frame.
    pub fn detect_objects(&self, frame: &FrameData) -> Vec<DetectedObject> {
        frame.objects.clone()
    }

    /// Track changes between two frames.
    pub fn track_changes(&mut self, previous: &FrameData, current: &FrameData) -> Vec<SceneChange> {
        let prev_counts = count_labels(&labels_of(&previous.objects));
        let curr_counts = count_labels(&labels_of(&current.objects));
        let mut changes = Vec::new();

        let all_labels: std::collections::HashSet<String> = prev_counts
            .keys()
            .chain(curr_counts.keys())
            .cloned()
            .collect();

        for label in all_labels {
            let prev_n = prev_counts.get(&label).copied().unwrap_or(0);
            let curr_n = curr_counts.get(&label).copied().unwrap_or(0);

            match curr_n.cmp(&prev_n) {
                std::cmp::Ordering::Greater => {
                    let desc = format!("{curr_n} {label}(s) appeared (was {prev_n})");
                    changes.push(SceneChange {
                        change_type: ChangeType::Appeared,
                        description: desc,
                        object_label: label,
                    });
                }
                std::cmp::Ordering::Less => {
                    let desc = format!("{label}(s) disappeared (from {prev_n} to {curr_n})");
                    changes.push(SceneChange {
                        change_type: ChangeType::Disappeared,
                        description: desc,
                        object_label: label,
                    });
                }
                std::cmp::Ordering::Equal => {}
            }
        }

        // Detect movement via position changes for same-label objects.
        for prev_obj in &previous.objects {
            if let Some(curr_obj) = current
                .objects
                .iter()
                .find(|o| o.label == prev_obj.label && positions_differ(&prev_obj.position, &o.position))
            {
                changes.push(SceneChange {
                    change_type: ChangeType::Moved,
                    description: format!(
                        "{} moved from ({:.0},{:.0}) to ({:.0},{:.0})",
                        prev_obj.label,
                        prev_obj.position.x,
                        prev_obj.position.y,
                        curr_obj.position.x,
                        curr_obj.position.y,
                    ),
                    object_label: prev_obj.label.clone(),
                });
            }
        }

        // Update baseline.
        self.baseline = curr_counts;

        changes
    }

    /// Generate alerts from the current scene.
    pub fn check_alerts(&self, scene: &SceneDescription) -> Vec<SceneAlert> {
        let mut alerts = Vec::new();
        let ts = scene.timestamp;

        if scene.summary.contains("empty scene") {
            return alerts;
        }

        if scene.motion_detected {
            alerts.push(SceneAlert {
                severity: Severity::Low,
                description: "Motion detected in scene.".into(),
                timestamp: ts,
            });
        }

        // High object count → medium alert.
        if scene.objects.len() > 5 {
            alerts.push(SceneAlert {
                severity: Severity::Medium,
                description: format!("High object count: {} objects detected.", scene.objects.len()),
                timestamp: ts,
            });
        }

        // Objects that sound security-relevant.
        let security_labels = ["person", "vehicle", "weapon", "unknown"];
        for obj in &scene.objects {
            let lower = obj.to_lowercase();
            if security_labels.iter().any(|s| lower.contains(s)) {
                alerts.push(SceneAlert {
                    severity: Severity::High,
                    description: format!("Potentially significant object detected: {obj}"),
                    timestamp: ts,
                });
            }
        }

        alerts
    }

    /// Apply privacy masks to a scene description, redacting masked regions.
    pub fn apply_privacy(&self, scene: &SceneDescription, masks: &[PrivacyMask]) -> SceneDescription {
        let mut filtered_summary = scene.summary.clone();
        let mut filtered_detail = scene.detail.clone();
        let mut filtered_objects = scene.objects.clone();

        for mask in masks {
            // Remove the mask label from summary and detail.
            let placeholder = "[REDACTED]";
            filtered_summary = filtered_summary.replace(&mask.label, placeholder);
            filtered_detail = filtered_detail.replace(&mask.label, placeholder);
            filtered_objects.retain(|o| o.to_lowercase() != mask.label.to_lowercase());
        }

        SceneDescription {
            summary: filtered_summary,
            detail: filtered_detail,
            objects: filtered_objects,
            motion_detected: scene.motion_detected,
            timestamp: scene.timestamp,
        }
    }

    /// Access the accumulated baseline.
    pub fn baseline(&self) -> &HashMap<String, usize> {
        &self.baseline
    }

    /// Expose config (for tests / downstream use).
    pub fn config(&self) -> &CameraConfig {
        &self.config
    }
}

// --- helpers ---

fn count_labels(labels: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for l in labels {
        *map.entry(l.clone()).or_insert(0) += 1;
    }
    map
}

fn labels_of(objects: &[DetectedObject]) -> Vec<String> {
    objects.iter().map(|o| o.label.clone()).collect()
}

fn positions_differ(a: &BoundingBox, b: &BoundingBox) -> bool {
    let threshold = 5.0;
    (a.x - b.x).abs() > threshold || (a.y - b.y).abs() > threshold
}
