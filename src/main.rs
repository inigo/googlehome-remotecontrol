extern crate iron;
extern crate rustc_serialize;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use std::process::Command;
use std::io::Read;
use std::fmt::{self, Formatter, Display};
use rustc_serialize::json::{self};

const BLACK_BEAN_PROGRAM: &str = "/Users/inigosurguy/Code/Inigo/rmmini/BlackBeanControl/BlackBeanControl.py";

fn main() {
    println!("Starting Rust listener");
    Iron::new(send_response).http("0.0.0.0:9000").unwrap();
}

fn send_response(request: &mut Request) -> IronResult<Response> {
    println!("Sending response");

    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let action = extract_action_from_json(payload.as_str());
    let activity_opt = convert_action_to_activity(action.as_str());
    println!("Got activity of {:?}", activity_opt);

    if let Some(ref a) = activity_opt {
        call_remote_control(a.remote_control_action.as_str() );
        println!("Called remote control with {}", a.remote_control_action);
    }


    let response_message = activity_opt.map(|a| a.message).unwrap_or("Action not known".to_string());

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Application/Json));
    // This would be done better with Serde
    response.set_mut(format!("{{ 'speech' : '{}', 'displayText' : '{}' }}", response_message, response_message));
    Ok(response)
}

fn extract_action_from_json(json_body: &str) -> String {
    // Note rustc_serialize is deprecated in favour of serde
    if let Ok(request_json) = json::Json::from_str(json_body) {
        if let Some(result) = request_json.find("result") {
            if let Some(action) = result.find("action") {
                let result = action.as_string().unwrap();
                return result.to_string();
            };
        };
    };
    "Unknown".to_owned()
}

fn convert_action_to_activity(action_name: &str) -> Option<Activity> {
    let activities = activity_lookup();
    activities.iter().find(|a| a.activity_name == action_name).map(|a| a.clone())
}

fn activity_lookup() -> Vec<Activity> {
    vec![
        Activity { activity_name: "TvPowerOn".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning on the tv".to_string() }
        , Activity { activity_name: "TvPowerOff".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning off the tv".to_string() }
        , Activity { activity_name: "SpeakerPowerOn".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning on the speaker".to_string() }
        , Activity { activity_name: "SpeakerPowerOff".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning off the speaker".to_string() }
    ]
}

fn call_remote_control(action_name: &str) {
    Command::new(BLACK_BEAN_PROGRAM)
        .args(&["-c", action_name])
        .output()
        .expect("failed to execute process");
}

// --------------

#[derive(Clone)]
#[derive(Debug)]
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
    assert_eq!( convert_action_to_activity( "TvPowerOn").unwrap().remote_control_action, "TvPower");
    assert_eq!( convert_action_to_activity("TvPowerOn").unwrap().message, "Turning on the tv");
    assert_eq!( convert_action_to_activity("TvPowerOff").unwrap().message, "Turning off the tv");
    assert_eq!( convert_action_to_activity("SpeakerPowerOff").unwrap().remote_control_action, "SpeakerPower")
}