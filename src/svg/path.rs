use super::FieldBorder;

impl FieldBorder {
    pub(super) fn to_svg_path_at_angle(self, degrees: f64) -> svg::node::element::Path {
        svg::node::element::Path::new()
            .set("fill", "black")
            .set("d", self.path_data)
            .set("transform", format!("rotate({}, 0, 0)", degrees))
    }
}