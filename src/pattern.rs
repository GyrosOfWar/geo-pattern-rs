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
    hash: String
}

impl Pattern {
    fn new(x_size: uint, y_size: uint, hash: String) -> Pattern {
        Pattern {
            svg: Svg::new(x_size, y_size),
            hash: hash
        }
    }
}
