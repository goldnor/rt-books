fn main() {
    let front_face = if dot(ray_direction, outward_normal) > 0.0 {
        // ray is inside the sphere
        normal = -outward_normal;
        false
    } else {
        // ray is outside the sphere
        normal = outward_normal;
        true
    };
}
