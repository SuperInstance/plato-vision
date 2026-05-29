# plato-vision — Camera → Text Scene Analysis

Translate camera frames into text descriptions for agents. Object detection, scene description, change tracking, and alerting — all as structured, human-readable text.

**Part of the [Plato](https://github.com/SuperInstance/plato-shell) ecosystem.**

## What This Gives You

- **Scene descriptions** — summary, detail, detected objects, motion status
- **Object detection** — label, confidence, bounding box, attributes
- **Change tracking** — appeared, disappeared, moved, changed between frames
- **Alerting** — severity-ranked alerts (Low/Medium/High/Critical) for scene changes
- **Baseline tracking** — accumulate expected state, detect anomalies

## Quick Start

```rust
use plato_vision::{Vision, CameraConfig, FrameData, DetectedObject, BoundingBox};

let vision = Vision::new(CameraConfig::default());

// Describe a frame
let frame = FrameData {
    objects: vec![
        DetectedObject { label: "person".into(), confidence: 0.95, position: BoundingBox { x: 100.0, y: 200.0, w: 80.0, h: 200.0 }, attributes: vec!["standing".into()] },
    ],
    motion_detected: true,
    timestamp: now_ms(),
};

let desc = vision.describe_frame(&frame);
println!("Scene: {}", desc.summary);
// "Scene containing person."

// Detect changes between frames
let changes = vision.track_changes(&previous_frame, &frame);
for change in changes {
    println!("{:?}: {}", change.change_type, change.description);
}
```

## How It Fits

Produces the "vision shadows" that flow through the system. [plato-transport](https://github.com/SuperInstance/plato-transport) delivers them to [plato-correlator](https://github.com/SuperInstance/plato-correlator), which fuses them with [plato-sonar-text](https://github.com/SuperInstance/plato-sonar-text) shadows. [plato-shell](https://github.com/SuperInstance/plato-shell) presents vision scenes as room descriptions.

## Installation

```toml
[dependencies]
plato-vision = "0.1"
```

## Testing

```bash
cargo test
```

## License

MIT
