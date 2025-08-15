[repo](https://github.com/Keavon/Bezier-rs) • [crates.io](https://crates.io/crates/bezier-rs) • [docs.rs](https://docs.rs/bezier-rs/latest/bezier_rs/) • [interactive docs](https://keavon.github.io/Bezier-rs/)

# Bezier-rs

Computational geometry algorithms for Bézier segments and paths useful in the context of 2D graphics.

Play with the interactive documentation which visualizes each API function in a fun, graphical manner:

### [**View the interactive API**](https://keavon.github.io/Bezier-rs/)

---

## Deprecation Notice

Bezier-rs was built for the needs of [Graphite](https://github.com/GraphiteEditor/Graphite), an open source 2D graphics editor. The library is now deprecated and has been archived in this repository by extracting its development history from Graphite's monorepo, with some cleanup in support of its standalone archival.

Graphite has moved to [Kurbo](https://github.com/linebender/kurbo) as of 2025, which offers superior performance and correctness compared to the naïve and unoptimized algorithms implemented here. However, some algorithms offered by Bezier-rs are not yet available in Kurbo. Note also that Bezier-rs is anchor-centric while Kurbo (like SVG) is segment-centric, meaning paths in Bezier-rs are defined by their anchor points and incoming/outgoing handles, while Kurbo paths are defined by segment commands like move-to, line-to, quadratic-to, and cubic-to.

No further development will continue and 0.5 is the last major version. Interested contributors are encouraged to submit missing algorithms to Kurbo instead.

## Features

Bezier-rs is inspired by [Bezier.js](https://pomax.github.io/bezierjs/) and [_A Primer on Bézier Curves_](https://pomax.github.io/bezierinfo/) by Pomax. It is not a port, so its API differs and additionally covers paths, not only single Bézier segments. View the [interactive documentation](https://keavon.github.io/Bezier-rs/) or the [docs.rs](https://docs.rs/bezier-rs/latest/bezier_rs/) documentation for more feature details.

## Terminology

Bezier-rs uses the following terminology for vector data. These depictions are given for cubic Bézier curves.

![Manipulators](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/manipulator-groups.png)
![Curve/Bezier Segment](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/curve-bezier-segment.png)
![Subpath/Path](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/subpath-path.png)
![Open/Closed](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/closed-open-subpath.png)
