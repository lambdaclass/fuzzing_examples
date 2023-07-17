pub mod c_json;
use c_json::{cJSON, cJSON_Parse, cJSON_GetObjectItemCaseSensitive};
use serde_json::Value;

fn main() {
    let json_string = "{\n\t\"name\":\"John\"\n}";
    let json = unsafe { cJSON_Parse(json_string.as_ptr() as *const i8) };
    let name_ptr: *mut cJSON = unsafe { cJSON_GetObjectItemCaseSensitive(json, "name".as_ptr() as *const i8).into() };
    let name: cJSON = unsafe { *name_ptr };
    let v: Value = serde_json::from_str(json_string).unwrap();
    println!("SERDE: {}", v["name"]);
    unsafe { println!("cJSON: {:?}", std::ffi::CStr::from_ptr(name.valuestring)) };
}
