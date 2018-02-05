extern crate iron;
extern crate rustc_serialize;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use std::process::Command;
use std::io::Read;
use std::fmt::{self, Formatter, Display};
use rustc_serialize::json::{self};

fn main() {
    println!("Starting Rust listener");
    Iron::new(send_response).http("0.0.0.0:9000").unwrap();
}

fn send_response(request: &mut Request) -> IronResult<Response> {
    println!("sending response");

    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let action = extract_action_from_json(payload.as_str());
    let activity = convert_action_to_activity(action.as_str());
    println!("Got activity of {}", activity);

    call_remote_control(activity.remote_control_action.as_str());

    println!("Called remote control with {}", activity.remote_control_action);

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Application/Json));
    // This would be done better with Serde
    response.set_mut(format!(r#"
        {{ "speech" : "{}", "displayText" : "{}" }}
    "#, activity.message, activity.message));
    Ok(response)
}

fn extract_action_from_json(json_body: &str) -> String {
    // Note rustc_serialize is deprecated in favour of serde
    if let Ok(request_json) = json::Json::from_str(json_body) {
        if let Some(result) = request_json.find("result") {
            if let Some(action) = result.find("action") {
                // returning String here not &str for reasons of ownership I don't yet really understand
                let result = action.as_string().unwrap();
                return result.to_owned();
            };
        };
    };
    "Unknown".to_owned()
}

fn convert_action_to_activity(action_name: &str) -> Activity {
    if action_name=="TvPowerOn" {
        Activity { activity_name: "TvPowerOn".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning on the tv".to_string() }
    } else if action_name=="TvPowerOff" {
        Activity { activity_name: "TvPowerOff".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning off the tv".to_string() }
    } else if action_name=="SpeakerPowerOn" {
        Activity { activity_name: "SpeakerPowerOn".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning on the speaker".to_string() }
    } else if action_name=="SpeakerPowerOff" {
        Activity { activity_name: "SpeakerPowerOff".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning off the speaker".to_string() }
    } else {
        Activity { activity_name: "Unknown".to_string(), remote_control_action: "Unknown".to_string(), message: "Action not known".to_string() }
    }
}

fn call_remote_control(action_name: &str) {
    Command::new("/Users/inigosurguy/Code/Inigo/rmmini/BlackBeanControl/BlackBeanControl.py")
        .args(&["-c", action_name])
        .output()
        .expect("failed to execute process");
}

// --------------

struct Activity {
    activity_name: String,
    remote_control_action: String,
    message: String
}

impl Display for Activity {
    // `f` is a buffer, this method must write the formatted string into it using write!
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ActivityName: {}, remote_control {}, message '{}'°",
               self.activity_name, self.remote_control_action, self.message)
    }
}

// --------------

#[test] fn test_extract_action_from_json() {
    let json_body = r#"{ "result": { "action": "TvPower" } }"#;
    assert_eq!( extract_action_from_json(json_body), "TvPower" )
}

#[test] fn test_call_remote_control() {
    call_remote_control("TvPower")
}

#[test] fn test_convert_action_to_activity() {
    assert_eq!( convert_action_to_activity("TvPowerOn").remote_control_action, "TvPower");
    assert_eq!( convert_action_to_activity("TvPowerOn").message, "Turning on the tv");
    assert_eq!( convert_action_to_activity("TvPowerOff").message, "Turning off the tv");
    assert_eq!( convert_action_to_activity("SpeakerPowerOff").remote_control_action, "SpeakerPower")
}