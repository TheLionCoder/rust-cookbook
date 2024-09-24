pub fn calculate_length_side_of_triangle() {
    let angle: f64 = 2.;
    let length_side = 80.0f64;

    let hypotenuse: f64 = length_side /angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}

pub fn verify_tan() {
    let x = 6.0f64;
    let a: f64 = x.tan();
    let b: f64 = x.sin() / x.cos();

    assert_eq!(a, b)
}

pub fn calculate_distance_between_two_points_on_earth() {
    let earth_radius_km = 6371.0_f64;
    let (paris_lat_deg, paris_long_deg) = (48.85341_f64, -2.34880_f64);
    let (london_lat_deg, london_long_deg) = (51.50853_f64, -0.12574_f64);

    let paris_lat_rad: f64 = paris_lat_deg.to_radians();
    let london_lat_rad: f64 = london_lat_deg.to_radians();

    let delta_lat: f64 = (paris_lat_deg - london_lat_deg).to_radians();
    let delta_long: f64 = (paris_long_deg - london_long_deg).to_radians();

    let central_angle_inner: f64 = (delta_lat / 2.).sin().powi(2)
        + paris_lat_rad.cos() * london_lat_rad.cos() * (delta_long / 2.).sin().powi(2);
    let central_angle: f64 = 2. * central_angle_inner.sqrt().asin();

    let distance: f64 = earth_radius_km * central_angle;

    println!(
        "Distance between Paris and london on the surface of the Earth is {:.1}",
        distance
    );
}
