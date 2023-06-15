// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

// Let's make something move! In this example, we'll see how to tell what a
// device can do, then send it a command (assuming it vibrates)!

use buttplug::{
  client::{ButtplugClientDevice, ButtplugClientEvent, VibrateCommand},
  util::in_process_client,
};
use futures::StreamExt;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use std::sync::Mutex;
use rand::distributions::{Distribution, Uniform};
use time::{PrimitiveDateTime, Time};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref CUR_PATTERN : Mutex<usize> = Mutex::new(1);
  static ref CUR_MODE : Mutex<usize> = Mutex::new(2);
  static ref ALARM_TIME : Mutex<Option<PrimitiveDateTime>> = Mutex::new(None);
  static ref CUR_VALUE : Mutex<f64> = Mutex::new(0.5);
  static ref STRENGTH : Mutex<f64> = Mutex::new(1.0);
}

fn now() -> PrimitiveDateTime {
  return PrimitiveDateTime::now() + time::Duration::hours(8)
}

fn pattern0(_time: i64) -> Vec<f64> {
  return Vec::from([0.0, 0.0]);
}

fn pattern1(_time: i64) -> Vec<f64> {
  return Vec::from([1.0, 1.0]);
}

fn pattern2(time: i64) -> Vec<f64> {
  let i = time % 10;
  return Vec::from([(i + 1) as f64 / 10.0, (10 - i) as f64 / 10.0]);
}

fn pattern3(time: i64) -> Vec<f64> {
  let s1 = [1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0];
  let s2 = [1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];
  let i: usize = (time as usize) % 32;
  return Vec::from([s1[i] as f64, s2[i] as f64]);
}

fn pattern4(time: i64) -> Vec<f64> {
  let s1 = [1,0,1,0,1,0,1,0,1,0];
  let i: usize = (time as usize) % 10;
  return Vec::from([s1[i] as f64, (i + 1) as f64 / 10.0]);
}

fn pattern5(time: i64) -> Vec<f64> {
  let i: usize = (time as usize) % 12;
  return Vec::from([if i < 10 { 0.2 } else { 1.0 }, 0.5]);
}

fn pattern6(time: i64) -> Vec<f64> {
  let s1 = [1,0,0,1,0,0,1,0,0,1,0,0,1,0,1,0];
  let s2 = [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0];
  let i: usize = (time as usize) % 16;
  return Vec::from([s1[i] as f64, s2[i] as f64]);
}

fn pattern7(time: i64) -> Vec<f64> {
  let s1 = [1,1,1,1,1,1,1,0,1,0,1,0,1,1,1,1,1,1,1,0,1,0,1,0];
  let s2 = [1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,];
  let i: usize = (time as usize) % 24;
  return Vec::from([if s1[i] == 1 {1.0} else {0.5}, if s2[i] == 1 {1.0} else {0.5}]);
}

fn pattern8(time: i64) -> Vec<f64> {
  let s1 = [1, 0, 0, 0, 0, 0];
  let s2 = [0, 1, 0, 0, 0, 0];
  let i: usize = (time as usize) % 6;
  return Vec::from([if s1[i] == 1 {1.0} else {0.2}, if s2[i] == 1 {1.0} else {0.2}]);
}

fn pattern9(time: i64) -> Vec<f64> {
  let i: usize = (time as usize) % 35 / 10;
  let s = [0.25, 0.5, 0.75, 1.0];
  return Vec::from([s[i], s[i]]);
}

fn pattern10(time: i64) -> Vec<f64> {
  let i: usize = (time as usize) % 3;
  let t: f64;
  if i == 0 {
    *CUR_VALUE.lock().unwrap() = (Uniform::from(0..10).sample(&mut rand::thread_rng()) as f64) / 10.0;
  }
  t = CUR_VALUE.lock().unwrap().clone();
  return Vec::from([t, t]);
}

async fn pattern_manager(ref cur_pattern : Arc<Mutex<usize>>) {
  loop {
    let mode = *CUR_MODE.lock().unwrap();
    if mode == 0 || mode == 1 { // idle
      *cur_pattern.lock().unwrap() = *CUR_PATTERN.lock().unwrap();
      sleep(Duration::from_secs(1)).await;
    } else if mode == 2 {  // random
      let pattern = Uniform::from(0..11).sample(&mut rand::thread_rng());
      let time = Uniform::from(2..15).sample(&mut rand::thread_rng());
      *cur_pattern.lock().unwrap() = pattern;
      sleep(Duration::from_secs(time)).await;
    }
  }
}

async fn controller(dev: Arc<ButtplugClientDevice>) {
  let patterns = Vec::from([pattern0, pattern1, pattern2, pattern3, pattern4, pattern5, pattern6, pattern7, pattern8, pattern9, pattern10]);
  let mut time = 0;
  let cur_pattern = Arc::new(Mutex::new(1));
  let cur_pattern_handle = Arc::clone(&cur_pattern);

  // then start a pattern manager
  tokio::spawn(async move {
    pattern_manager(cur_pattern_handle).await;
  });

  // control loop: motor = 2
  loop {
    let pattern = cur_pattern.lock().unwrap().clone();
    let strength = STRENGTH.lock().unwrap().clone();
    let mut vec = patterns[pattern](time);
    for i in &mut vec {
      *i *= strength;
    }

    if let Err(_) = dev.vibrate(&VibrateCommand::SpeedVec(vec)).await {
      println!("Device has 1 motor");
      break;
    };
    sleep(Duration::from_millis(100)).await;
    time += 1;
  }
  
  // control loop: motor = 1
  loop {
    let pattern = cur_pattern.lock().unwrap().clone();
    let strength = STRENGTH.lock().unwrap().clone();
    let mut vec = patterns[pattern](time);
    for i in &mut vec {
      *i *= strength;
    }

    if let Err(_) = dev.vibrate(&VibrateCommand::Speed(vec[0])).await {
      println!("Device disconnected, please reconnect!");
      return;
    };
    sleep(Duration::from_millis(100)).await;
    time += 1;
  }
}

async fn device_manager() {
  let client = in_process_client("Test Client", false).await;
  let mut event_stream = client.event_stream();

  if let Err(err) = client.start_scanning().await {
    println!("Client errored when starting scan! {}", err);
    return;
  }

  let connect_and_start_device = |dev: Arc<ButtplugClientDevice>| {
    async move {
      let name = dev.name();
      
      // Test it
      if dev.message_attributes().scalar_cmd().is_some() {
        if let Err(e) = dev.vibrate(&VibrateCommand::Speed(1.0)).await {
          println!("Error sending vibrate command to device! {}", e);
          return;
        }
        sleep(Duration::from_secs(1)).await;
        if let Err(e) = dev.stop().await {
          println!("Error sending stop command to device! {}", e);
          return;
        }
        sleep(Duration::from_secs(1)).await;
        println!("{} connected succcessfully. Enjoy!", name);

        tokio::spawn(async move {
          controller(dev).await;
        });

      } else {
        println!("{} doesn't vibrate! This example should be updated to handle rotation and linear movement!", name);
      }
    }
  };

  loop {
    match event_stream
      .next()
      .await
      .expect("We own the client so the event stream shouldn't die.")
    {
      ButtplugClientEvent::DeviceAdded(dev) => {
        println!("We got a device: {}", dev.name());
        tokio::spawn(async move {
          connect_and_start_device(dev).await;
        });
        // break;
      }
      ButtplugClientEvent::ServerDisconnect => {
        // The server disconnected, which means we're done here, so just
        // break up to the top level.
        println!("Server disconnected!");
        break;
      }
      _ => {
        // Something else happened, like scanning finishing, devices
        // getting removed, etc... Might as well say something about it.
        println!("Got some other kind of event we don't care about");
      }
    }
  }

}

async fn input_manager() {
  loop {
    let mut line = String::new();
    let len = std::io::stdin().read_line(&mut line).unwrap();
    let vec = line[..len-2].split(' ').collect::<Vec<&str>>();
    println!("{:?}", vec);
    if vec.len() == 0 {
      continue;
    }
    else if vec.len() == 1 {
      if vec[0] == "pause" {
        *CUR_MODE.lock().unwrap() = 0;
        *CUR_PATTERN.lock().unwrap() = 0;
        println!("paused.")
      }
      else if vec[0] == "high" {
        *CUR_MODE.lock().unwrap() = 1;
        *CUR_PATTERN.lock().unwrap() = 1;
        println!("high.")
      }
      else if vec[0] == "random" {
        *CUR_MODE.lock().unwrap() = 2;
        println!("start random, enjoy.")
      }
      else if vec[0] == "show_alarm" {
        let _ = match *ALARM_TIME.lock().unwrap() {
          Some(t) => {
            println!("alarm set at {}", t);
          }
          None => {
            println!("no alarm set");
          }
        };
      }
    }
    else if vec.len() == 2 {
      if vec[0] == "pattern" {
        *CUR_MODE.lock().unwrap() = 1;
        let pattern = vec[1].parse::<usize>().unwrap();
        *CUR_PATTERN.lock().unwrap() = pattern;
        println!("pattern changed to {}.", pattern);
      }
      else if vec[0] == "strength" {
        let strength = vec[1].parse::<f64>().unwrap();
        *STRENGTH.lock().unwrap() = strength;
        println!("strength changed to {}.", strength);
      }
      else if vec[0] == "alarm" {
        let mut t = match Time::parse(vec[1], "%T") {
          Ok(t) => {
            let now = now().date();
            now.with_time(t)
          },
          Err(e) => {
            println!("error: {}", e);
            continue;
          }
        };
        println!("{} vs {}", t, now());
        if t <= now() {
          println!("set to tomorrow.");
          t += time::Duration::hours(24);
        }
        
        *ALARM_TIME.lock().unwrap() = Some(t);
        println!("alarm set at {:?}", t);
      }
    }
  }
}

async fn alarm_manager() {
  loop {
    let alarm = *ALARM_TIME.lock().unwrap();
    let _ = match alarm {
      Some(t) => {
        if t <= now() {
          *CUR_MODE.lock().unwrap() = 2;
          println!("alarm time reached, start random, enjoy.");
          *ALARM_TIME.lock().unwrap() = None;
        }
      }
      None => {}
    };
  }
}

#[tokio::main]
async fn main() {
  // start a device manager
  tokio::spawn(async move {
    device_manager().await;
  });

  // then start a input manager
  tokio::spawn(async move {
    input_manager().await;
  });

  // then start a alarm manager
  tokio::spawn(async move {
    alarm_manager().await;
  });

  loop {

  }
}
