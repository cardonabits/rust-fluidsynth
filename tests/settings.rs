extern crate fluidsynth;
use fluidsynth::settings::*;

#[test]
fn setstr() {
    let settings = Settings::new();
    // false means success
    assert_eq!(settings.setstr("audio.driver", "alsa"), false);
}

#[test]
fn get_type() {
    let settings = Settings::new();
    settings.setstr("audio.driver", "alsa");
    assert_eq!(settings.get_type("audio.driver"), SettingsType::StrType);
}

#[test]
fn get_type_unknown_setting() {
    let settings = Settings::new();
    assert_eq!(settings.get_type("unknown setting"), SettingsType::NoType);
}

#[ignore]  // not implemented
#[test]
fn copystr() {
    let settings = Settings::new();
    settings.setstr("audio.driver", "coreaudio");
    let mut buffer: String = String::with_capacity(12);
// not implemented
//    let res = settings.copystr("audio.driver", &mut buffer, 12);
    assert_eq!(buffer, "coreaudio");
//    assert_eq!(res, true);
}

#[ignore]
#[test]
fn getstr() {
    let settings = Settings::new();
    settings.setstr("audio.driver", "coreaudio");
// not implemented
//    let res = settings.getstr("audio.driver");
//    assert_eq!(res.unwrap(), "coreaudio");
}

#[ignore]
#[test]
fn getstr_default_unknown_setting() {
    let settings = Settings::new();
    let res = settings.getstr_default("unknown setting");
    assert_eq!(res, None);
}

#[ignore]
#[test]
fn getstr_default() {
    let settings = Settings::new();
    let res = settings.getstr_default("audio.coreaudio.device");
    assert_eq!(res, Some("default".to_string()));
}

#[ignore]
#[test]
fn getstr_equal() {
    let settings = Settings::new();
    settings.setstr("audio.driver", "alsa");
    assert!(settings.getstr_equal("audio.driver", "alsa"));
}

#[ignore]
#[test]
fn getstr_equal_fail() {
    let settings = Settings::new();
    settings.setstr("audio.driver", "coreaudio");
    //assert!(settings.getstr_equal("audio nonextising", "foo"));
}

#[test]
fn setnum() {
    let settings = Settings::new();
    assert!(settings.setnum("synth.sample-rate", 44000.00) == false);
}

#[test]
fn getnum() {
    let settings = Settings::new();
    settings.setnum("synth.sample-rate", 44000.00);
    assert_eq!(settings.getnum("synth.sample-rate"), Some(44000.00));
}

#[test]
fn getnum_fail() {
    let settings = Settings::new();
    assert_eq!(settings.getnum("no setting"), None);
}

#[test]
fn getnum_default_none() {
    let settings = Settings::new();
    assert_eq!(settings.getnum_default("no setting"), None);
}

#[test]
fn getnum_default() {
    let settings = Settings::new();
    assert_eq!(settings.getnum_default("synth.overflow.volume"), Some(500.0));
}

#[test]
fn getnum_range() {
    let settings = Settings::new();
    assert_eq!(settings.getnum_range("synth.overflow.volume"), Some((-10000.00, 10000.00)));
}

#[test]
fn getnum_range_no_existing_setting() {
    let settings = Settings::new();
    assert_eq!(settings.getnum_range("no setting"), None);
}

#[test]
fn setint() {
    let settings = Settings::new();
    assert!(settings.setint("audio.periods", 4) == false);
}

#[test]
fn getint() {
    let settings = Settings::new();
    settings.setint("synth.min-note-length", 4);
    assert_eq!(settings.getint("synth.min-note-length"), Some(4));
}

#[test]
fn getint_fail() {
    let settings = Settings::new();
    assert_eq!(settings.getint("no setting"), None);
}

#[test]
fn getint_default_none() {
    let settings = Settings::new();
    assert_eq!(settings.getint_default("no setting"), None);
}

#[test]
fn getint_default() {
    let settings = Settings::new();
    assert_eq!(settings.getint_default("audio.channels"), None);
}

#[test]
fn getint_range() {
    let settings = Settings::new();
    assert_eq!(settings.getint_range("synth.effects-channels"), Some((2, 2)));
}

#[test]
fn getint_range_no_existing_setting() {
    let settings = Settings::new();
    assert_eq!(settings.getint_range("no setting"), None);
}

fn foreach_option_callback(name: &str, option: &str) {
    println!("{}", name);
    println!("{}", option);
}

#[test]
fn foreach_option() {
    let settings = Settings::new();
    settings.foreach_option("audio.driver", foreach_option_callback);
}

#[test]
fn option_count() {
    let settings = Settings::new();
    assert!(settings.option_count("audio.driver").is_some());
}

#[test]
fn option_count_not_a_str() {
    let settings = Settings::new();
    assert_eq!(settings.option_count("synth.effects-channels"), None);
}

#[test]
fn option_concat() {
    let settings = Settings::new();
    assert!(settings.option_concat("audio.driver", ",").is_some());
}

#[test]
fn option_concat_fail() {
    let settings = Settings::new();
    assert_eq!(settings.option_concat("no such type", ","), None);
}
