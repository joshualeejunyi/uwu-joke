#![windows_subsystem = "windows"]

use clipboard::{ClipboardContext, ClipboardProvider};
use winput::{Vk, Action};
use winput::message_loop;
use std::process;
use std::{thread, time};

const NUM_OF_SPACES: i32 = 10;

fn cut_text(num_of_back: i32) {
    println!("cutting... {}", num_of_back);
    winput::press(Vk::Control);
    winput::press(Vk::Shift);

    for _ in 0..num_of_back {
        winput::press(Vk::LeftArrow);
        winput::release(Vk::LeftArrow);
    }

    winput::release(Vk::Shift);
    winput::release(Vk::Control);

    winput::press(Vk::Control);
    winput::press(Vk::X);
    println!("end cutting...");
}

fn paste_text() {
    winput::press(Vk::Control);
    winput::press(Vk::V);
    winput::release(Vk::Control);
    winput::release(Vk::V);
}

fn main() {
    let receiver = message_loop::start().unwrap();

    println!("hello world")

    let delay = time::Duration::from_millis(15);
    let mut num = 0;
    let mut punc_count = 0;
    // initialize the prev key as space first cause idk how to do otherwise.
    let mut previous_key = Vk::Space;
    let mut clipboard: ClipboardContext = ClipboardProvider::new().expect("Failed to create clipboard context");
        
    let punctuation_list = [
        Vk::Period,
        Vk::Comma,
        Vk::Plus,
        Vk::Minus,
        Vk::Oem1,
        Vk::Oem2,
        Vk::Oem3,
        Vk::Oem4,
        Vk::Oem5,
        Vk::Oem6,
        Vk::Oem7,
    ];

    let num_list = [
        Vk::_0,
        Vk::_1,
        Vk::_2,
        Vk::_4,
        Vk::_5,
        Vk::_6,
        Vk::_7,
        Vk::_8,
        Vk::_9,
        Vk::Oem1,
        Vk::Oem2,
        Vk::Oem3,
        Vk::Oem4,
        Vk::Oem5,
        Vk::Oem6,
        Vk::Oem7,
    ];

    loop {
        let mut num_of_back = NUM_OF_SPACES;

        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                println!("Detected: {:?}", &vk);

                // if a chunk of text is copied, uwuify it
                if previous_key == Vk::Control {
                    if vk == Vk::C {
                        thread::sleep(delay);
                        if let Ok(contents) = clipboard.get_contents() {
                            clipboard
                                .set_contents(uwuifier::uwuify_str_sse(contents.as_str()))
                                .expect("Failed to set clipboard contents");
                        }
                    }

                // check for punctuation
                } else if previous_key == Vk::Shift {
                    if num_list.contains(&vk) || punctuation_list.contains(&vk) {
                        punc_count += 1;
                    }
                }

                if punctuation_list.contains(&vk) {
                    punc_count += 1;
                    
                } else if vk == Vk::Space {
                    if num == NUM_OF_SPACES - 1 {
                        num_of_back += punc_count;

                        cut_text(num_of_back);

                        // delay cause this is somehow too fast LOL
                        thread::sleep(delay);

                        if let Ok(contents) = clipboard.get_contents() {
                            println!("clipboard: {:?}", contents);
                            clipboard
                            .set_contents(uwuifier::uwuify_str_sse(contents.as_str()))
                            .expect("Failed to set clipboard contents");

                            println!("post-clipboard: {:?}", clipboard.get_contents());
                        }
                        // delay 
                        thread::sleep(delay);
                        paste_text();
                        // winput::send_inputs(&paste_text);



                        num = 0;
                        punc_count = 0;
                    } else {
                        num += 1;
                    }

                // print screen as the killswitch
                } else if vk == Vk::PrintScreen {
                    process::exit(0);
                }

                previous_key = vk;
            },
            _ => (),
        }
    }
}
