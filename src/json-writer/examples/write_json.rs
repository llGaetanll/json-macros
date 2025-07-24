use json_traits::Serialize;
use json_writer::json;

fn main() {
    let mut buf = Vec::new();

    let name = String::from("Alice Johnson");
    let age = 29u32;
    let active = true;
    let height = 5.7f64;
    let scores = vec![85, 92, 78, 96, 88];
    let tags = ["developer", "rust", "backend"];
    let coordinates = [40.7128, -74.0060];
    let metadata = vec![true, false, true];

    json!(&mut buf, {
        name,
        age,
        active,
        height,
        test_scores: scores,
        skills: tags,
        location: {
            coordinates,
            city: "New York",
            country: "USA"
        },
        flags: metadata,
        team_size: 12u16,
        salary: 85000.50f32,
        remote: false
    });

    println!("{}", String::from_utf8_lossy(&buf));

    let exp = br#"{"name":"Alice Johnson","age":29,"active":true,"height":5.7,"test_scores":[85,92,78,96,88],"skills":["developer","rust","backend"],"location":{"coordinates":[40.7128,-74.006],"city":"New York","country":"USA"},"flags":[true,false,true],"team_size":12,"salary":85000.50,"remote":false}"#;
    assert_eq!(
        buf,
        exp,
        "\nl: {}\nr: {}",
        String::from_utf8_lossy(&buf),
        String::from_utf8_lossy(exp)
    );
}
