use plato_vision::*;
use plato_vision::vision::Vision;

fn bb(x: f32, y: f32) -> BoundingBox {
    BoundingBox { x, y, w: 100.0, h: 100.0 }
}

fn obj(label: &str, confidence: f32, x: f32, y: f32) -> DetectedObject {
    DetectedObject {
        label: label.into(),
        confidence,
        position: bb(x, y),
        attributes: vec![],
    }
}

#[test]
fn describe_frame_produces_text_summary() {
    let vision = Vision::new(CameraConfig::default());
    let frame = FrameData {
        objects: vec![
            obj("chair", 0.9, 10.0, 20.0),
            obj("table", 0.85, 50.0, 60.0),
        ],
        motion_detected: true,
        timestamp: 1000,
    };
    let desc = vision.describe_frame(&frame);
    assert!(desc.summary.contains("chair"));
    assert!(desc.summary.contains("table"));
    assert!(desc.detail.contains("chair"));
    assert!(desc.motion_detected);
}

#[test]
fn detect_objects_finds_labeled_objects() {
    let vision = Vision::new(CameraConfig::default());
    let frame = FrameData {
        objects: vec![
            obj("cat", 0.95, 10.0, 10.0),
            obj("dog", 0.80, 200.0, 200.0),
        ],
        motion_detected: false,
        timestamp: 1000,
    };
    let detected = vision.detect_objects(&frame);
    assert_eq!(detected.len(), 2);
    assert_eq!(detected[0].label, "cat");
    assert_eq!(detected[1].label, "dog");
}

#[test]
fn track_changes_detects_new_objects() {
    let mut vision = Vision::new(CameraConfig::default());
    let prev = FrameData::empty();
    let curr = FrameData {
        objects: vec![obj("person", 0.9, 50.0, 50.0)],
        motion_detected: true,
        timestamp: 2000,
    };
    let changes = vision.track_changes(&prev, &curr);
    assert!(changes.iter().any(|c| c.change_type == ChangeType::Appeared && c.object_label == "person"));
}

#[test]
fn track_changes_detects_disappeared_objects() {
    let mut vision = Vision::new(CameraConfig::default());
    let prev = FrameData {
        objects: vec![obj("car", 0.9, 100.0, 100.0)],
        motion_detected: true,
        timestamp: 1000,
    };
    let curr = FrameData::empty();
    let changes = vision.track_changes(&prev, &curr);
    assert!(changes.iter().any(|c| c.change_type == ChangeType::Disappeared && c.object_label == "car"));
}

#[test]
fn check_alerts_generates_text_for_high_severity() {
    let vision = Vision::new(CameraConfig::default());
    let scene = SceneDescription {
        summary: "Scene containing person, chair.".into(),
        detail: "person at (10,20); chair at (50,60)".into(),
        objects: vec!["person".into(), "chair".into()],
        motion_detected: true,
        timestamp: 1000,
    };
    let alerts = vision.check_alerts(&scene);
    assert!(alerts.iter().any(|a| a.severity == Severity::High));
    assert!(alerts.iter().any(|a| a.description.contains("person")));
}

#[test]
fn apply_privacy_removes_masked_regions() {
    let vision = Vision::new(CameraConfig::default());
    let scene = SceneDescription {
        summary: "Scene containing person, dog.".into(),
        detail: "person at (10,20)".into(),
        objects: vec!["person".into(), "dog".into()],
        motion_detected: false,
        timestamp: 1000,
    };
    let masks = vec![PrivacyMask {
        region: bb(10.0, 20.0),
        label: "person".into(),
    }];
    let redacted = vision.apply_privacy(&scene, &masks);
    assert!(redacted.summary.contains("[REDACTED]"));
    assert!(!redacted.objects.contains(&"person".to_string()));
    assert!(redacted.objects.contains(&"dog".to_string()));
}

#[test]
fn temporal_tracker_accumulates_baseline() {
    let mut vision = Vision::new(CameraConfig::default());
    let f1 = FrameData {
        objects: vec![obj("cat", 0.9, 10.0, 10.0)],
        motion_detected: true,
        timestamp: 1000,
    };
    vision.track_changes(&FrameData::empty(), &f1);
    assert_eq!(vision.baseline().get("cat"), Some(&1));

    let f2 = FrameData {
        objects: vec![obj("cat", 0.9, 10.0, 10.0), obj("cat", 0.8, 200.0, 200.0)],
        motion_detected: true,
        timestamp: 2000,
    };
    vision.track_changes(&f1, &f2);
    assert_eq!(vision.baseline().get("cat"), Some(&2));
}

#[test]
fn empty_frame_produces_empty_scene_description() {
    let vision = Vision::new(CameraConfig::default());
    let frame = FrameData::empty();
    let desc = vision.describe_frame(&frame);
    assert_eq!(desc.summary, "empty scene");
    assert!(desc.objects.is_empty());
    assert!(!desc.motion_detected);
}
