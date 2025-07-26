use json_reader::JsonColon;
use json_reader::JsonKey;
use json_reader::JsonObj;
use json_reader::JsonObjStart;
use json_reader::JsonResult;
use json_reader::JsonStr;
use json_reader::Parse;

fn main() -> JsonResult<()> {
    let input = br#"{"name":"Alice Johnson","age":29,"active":true,"height":5.7,"test_scores":[85,92,78,96,88],"skills":["developer","rust","backend"],"location":{"coordinates":[40.7128,-74.006],"city":"New York","country":"USA"},"flags":[true,false,true],"team_size":12,"salary":85000.50,"remote":false}"#.as_slice();

    let (_, input) = JsonObj::parse(input)?;

    println!("{}", String::from_utf8_lossy(input));

    Ok(())
}
