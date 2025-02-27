/*
 * (c) 2021 QVLX LLC. All rights reserved.
 *
 * !! WARNING !!
 * THIS DOCUMENT CONTAINS CONFIDENTIAL AND PROPRIETARY DATA ORIGINATED BY QVLX LLC.
 * ALL DESIGN, MANUFACTURING PRODUCTION, USE, SALE AND PATENT RIGHTS ARE EXPRESSLY RESERVED.
 *
 * THE DATA CONTAINED IN THIS DOCUMENT IS SUBJECT TO ITAR/EAR RESTRICTIONS.
 *
 * THE RECIPIENT AGREES BY VIEWING THIS DOCUMENT NOT TO SUPPLY OR DISCLOSE ANY INFORMATION
 * REGARDING IT TO UNAUTHORIZED PERSONS OR INCORPORATE INTO ANY OTHER DESIGN OR USE THEREOF.
 *
 * VIOLATION OF THESE TERMS WILL BE SUBJECT TO PROSECUTION AT THE FULL EXTENT OF THE LAW.
 */

/*
 * QVLx Salvum 
 *
 * debug.rs -> orchestrating program for Salvum
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

// Imports
use std::collections::HashMap;

// External files
use crate::events;

#[cfg(debug_assertions)]
pub static DEBUG_MODE: bool = true;
#[cfg(not(debug_assertions))]
pub static DEBUG_MODE: bool = false;

pub fn print_debug<D: std::fmt::Display>(msg: D) {
  if DEBUG_MODE {
    println!("{}", msg);
  }
}

#[allow(dead_code)]
pub fn print_all_events_map(events_map: HashMap<String, usize>) {
  for (event_name, event_id) in &events_map {
    println!("event name: {}\nevent id: {}", event_name, event_id);
  }
}

#[allow(dead_code)]
pub fn print_all_events_vec(events_vec: Vec<Box<dyn events::Eventable>>) {
  for event_box in &events_vec {
    let event: &events::Event = event_box.get_event();
    print_event(event);
  }
}

#[allow(dead_code)]
pub fn print_event(event: &events::Event) {
  println!("event name: {}", event.name);
  println!("event desc: {}", event.desc);
  print_links(&event.links);
}

#[allow(dead_code)]
pub fn print_links(links: &Vec<String>) {
  print!("links: ");
  for link in links {
    print!("{} ", link);
  }
  println!("");
}

#[allow(dead_code)]
pub fn print_mode(mode: u32) {
  match mode {
    0 => println!("Yellow mode"),
    1 => println!("Red mode"),
    2 => println!("Blue mode"),
    _ => println!("unrecognized mode"),
  }
}
