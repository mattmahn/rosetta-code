const EARTH_RADIUS: f64 = 6371.0;

fn main() {
    println!("{}", haversine(36.12, -86.67, 33.94, -118.40));
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = lat2.to_radians() - lat1.to_radians();
    let d_lon = lon2.to_radians() - lon1.to_radians();

    2.0 * EARTH_RADIUS *
    (((d_lat / 2.0).sin().powi(2) +
      lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2))
            .sqrt())
        .asin()
}
