extern crate iron;
extern crate rustc_serialize;
#[macro_use] extern crate mime;
#[macro_use] extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use std::io::Read;
use rustc_serialize::json::{self};
use std::borrow::Borrow;

use activities::convert_action_to_activity;
use remote_control::call_remote_control;

fn main() {
    println!("Starting Rust listener");
    Iron::new(send_response).http("0.0.0.0:9000").unwrap();
}

fn send_response(request: &mut Request) -> IronResult<Response> {
    println!("Sending response");

    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let action = extract_action_from_json(payload.as_str()).unwrap_or("Unknown".to_string());
    let activity_opt = convert_action_to_activity(action.as_str());
    println!("Got activity of {:?}", activity_opt);

    if let Some(ref a) = activity_opt {
        call_remote_control(a.remote_control_action.as_str() );
        println!("Called remote control with {}", a.remote_control_action);
    }


    let response_message: &str = activity_opt.map(|a| a.message.borrow()).unwrap_or("Action not known");

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Application/Json));
    // This would be done better with Serde
    response.set_mut(format!("{{ 'speech' : '{}', 'displayText' : '{}' }}", response_message, response_message));
    Ok(response)
}

fn extract_action_from_json(json_body: &str) -> Option<String> {
    // Note rustc_serialize is deprecated in favour of serde
    if let Ok(request_json) = json::Json::from_str(json_body) {
        let result= request_json.find("result")?;
        let action = result.find("action")?;
        let result = action.as_string().map(|a| a.to_string());
        return result;
    } else {
        None
    }
}

mod activities {
    use std::fmt::{self, Formatter, Display};

    lazy_static! {
        static ref ACTIVITIES : Vec<Activity> = vec![
            Activity { activity_name: "TvPowerOn".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning on the tv".to_string() }
            , Activity { activity_name: "TvPowerOff".to_string(), remote_control_action: "TvPower".to_string(), message: "Turning off the tv".to_string() }
            , Activity { activity_name: "SpeakerPowerOn".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning on the speaker".to_string() }
            , Activity { activity_name: "SpeakerPowerOff".to_string(), remote_control_action: "SpeakerPower".to_string(), message: "Turning off the speaker".to_string() }
            , Activity { activity_name: "TvSource".to_string(), remote_control_action: "TvSource".to_string(), message: "Changing the tv source".to_string() }
        ];
    }

    pub fn convert_action_to_activity(action_name: &str) -> Option<&Activity> {
        ACTIVITIES.iter().find(|a| a.activity_name == action_name)
    }

    #[derive(Debug)]
    pub struct Activity {
        pub activity_name: String,
        pub remote_control_action: String,
        pub message: String
    }

    impl Display for Activity {
        // `f` is a buffer, this method must write the formatted string into it using write!
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "ActivityName: {}, remote_control {}, message '{}'°",
                   self.activity_name, self.remote_control_action, self.message)
        }
    }

}

mod remote_control {
    use std::process::Command;

    const BLACK_BEAN_PROGRAM: &str = "BlackBeanControl.py";

    pub fn call_remote_control(action_name: &str) {
        Command::new(BLACK_BEAN_PROGRAM)
            .args(&["-c", action_name])
            .output()
            .expect("failed to execute process");
    }

    #[test] fn test_call_remote_control() {
        call_remote_control("TvPower")
    }

}

// --------------

#[test] fn test_extract_action_from_json() {
    let json_body = r#"{ "result": { "action": "TvPower" } }"#;
    assert_eq!( extract_action_from_json(json_body), Some("TvPower".to_string()) )
}

#[test] fn test_convert_action_to_activity() {
    assert_eq!( convert_action_to_activity( "TvPowerOn").unwrap().remote_control_action, "TvPower");
    assert_eq!( convert_action_to_activity("TvPowerOn").unwrap().message, "Turning on the tv");
    assert_eq!( convert_action_to_activity("TvPowerOff").unwrap().message, "Turning off the tv");
    assert_eq!( convert_action_to_activity("SpeakerPowerOff").unwrap().remote_control_action, "SpeakerPower")
}