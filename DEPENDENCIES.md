# Dependencies — plato-vision

## Ecosystem Role

plato-vision is the **observability and correlation** subsystem of Plato. It provides real-time visualization, metric correlation, and event stream processing for monitoring the health and behavior of agents, fleets, and constraint pipelines across the SuperInstance ecosystem.

---

## Upstream Dependencies

| Repository | Description |
|---|---|
| [openconstruct-abi](https://github.com/SuperInstance/openconstruct-abi) | ABI types for metric and event serialization |
| [plato-tick](https://github.com/SuperInstance/plato-tick) | Tick streams provide the raw data feed |
| [plato-adapters](https://github.com/SuperInstance/plato-adapters) | Adapter interfaces for visual output sinks |
| [plato-construct](https://github.com/SuperInstance/plato-construct) | Construct-level abstractions for dashboard layout |

## Downstream Dependents

| Repository | Description |
|---|---|
| [cocapn](https://github.com/SuperInstance/cocapn) | Agent coordination uses vision for dashboarding |
| [cocapn-health-rs](https://github.com/SuperInstance/cocapn-health-rs) | Health module renders via vision |
| [fleet-health-monitor](https://github.com/SuperInstance/fleet-health-monitor) | Fleet health visualizations |

## Documentation

- [OpenConstruct Docs](https://github.com/SuperInstance/openconstruct-docs)
- [SuperInstance Wiki](https://github.com/SuperInstance/superinstance-wiki)
