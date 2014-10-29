use svg::Svg;

enum PatternType {
    Octogons,
    OverlappingCircles,
    PlusSigns,
    Xes,
    SineWaves,
    Hexagons,
    OverlappingRings,
    Plaid,
    Triangles,
    Squares,
    ConcentricCircles,
    Diamonds,
    Tessellation,
    NestedSquares,
    MosaicSquares,
    Chevrons
}

struct Pattern {
    svg: Svg,
    hash: String,
    pattern_type: PatternType,
    // TODO color, background color
}

impl Pattern {
    fn new(x_size: uint, y_size: uint, hash: String, pattern_type: PatternType) -> Pattern {
        Pattern {
            svg: Svg::new(x_size, y_size),
            hash: hash,
            pattern_type: pattern_type
        }
    }
}
