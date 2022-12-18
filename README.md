# Bezier-rs

Computational geometry algorithms for Bézier segments and shapes useful in the context of 2D graphics.

Play with the [interactive documentation](https://keavon.github.io/Bezier-rs/) which visualizes each API function in a fun manner.

Bezier-rs is built for the needs of [Graphite](https://github.com/GraphiteEditor/Graphite), an open source 2D vector graphics editor. We hope it may be useful to others, but presently Graphite is its primary user. Pull requests are welcomed for new features, code cleanup, ergonomics enhancements, performance improvements, and documentation clarifications.

The library currently provides functions dealing with single Bézier curve segments, as well as open-or-closed multi-segment paths (which we call _subpaths_). In the future, the library will be expanded to include compound paths (multiple subpaths forming a single shape, where the winding order determines inside-or-outside-ness) and operations between paths (e.g. boolean functions).

Bezier-rs is inspired by [Bezier.js](https://pomax.github.io/bezierjs/) and [_A Primer on Bézier Curves_](https://pomax.github.io/bezierinfo/) by Pomax. Bezier-rs is not a port of Bezier.js so the API for single-segment Bézier curves has some differences, and the intention is to offer a broader scope that provides algorithms beyond single curve segments (as noted above) to eventually service full vector shapes.

## Terminology

Bezier-rs uses the following terminology for vector data. These depictions are given for cubic Bézier curves.

![Manipulators](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/manipulator-groups.png)
![Curve/Bezier Segment](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/curve-bezier-segment.png)
![Subpath/Path](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/subpath-path.png)
![Open/Closed](https://raw.githubusercontent.com/Keavon/Bezier-rs/refs/heads/master/interactive-docs/images/closed-open-subpath.png)
