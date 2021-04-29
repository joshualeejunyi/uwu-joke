#![windows_subsystem = "windows"]

use clipboard::{ClipboardContext, ClipboardProvider};
use winput::{Input, Vk, Action};
use winput::message_loop;
use std::process;
use std::{thread, time};

fn main() {
    let receiver = message_loop::start().unwrap();
    let delay = time::Duration::from_millis(25);
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
    
    let press_ctrlshift = [
        Input::from_vk(Vk::Control, Action::Press),
        Input::from_vk(Vk::Shift, Action::Press),
    ];
        
    let release_ctrlshift = [
        Input::from_vk(Vk::Shift, Action::Release),
        Input::from_vk(Vk::Control, Action::Release),
    ];

    let previous_word = [
        Input::from_vk(Vk::LeftArrow, Action::Press),
        Input::from_vk(Vk::LeftArrow, Action::Release),
    ];

    let cut_text = [
        Input::from_vk(Vk::Control, Action::Press),
        Input::from_vk(Vk::X, Action::Press),
        Input::from_vk(Vk::X, Action::Release),
        Input::from_vk(Vk::Control, Action::Release),
    ];

    let paste_text = [
        Input::from_vk(Vk::Control, Action::Press),
        Input::from_vk(Vk::V, Action::Press),
        Input::from_vk(Vk::V, Action::Release),
        Input::from_vk(Vk::Control, Action::Release),
    ];

    loop {
        let mut num_of_back = 3;

        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if previous_key == Vk::Shift {
                    if num_list.contains(&vk) || punctuation_list.contains(&vk) {
                        punc_count += 1;
                    }
                }

                if punctuation_list.contains(&vk) {
                    punc_count += 1;
                    
                } else if vk == Vk::Space {
                    if num == 2 {
                        winput::send_inputs(&press_ctrlshift);
                        num_of_back += punc_count;

                        for _ in 0..num_of_back {
                            winput::send_inputs(&previous_word);
                        }
                        winput::send_inputs(&release_ctrlshift);
                        winput::send_inputs(&cut_text);
                        // delay cause this is somehow too fast LOL
                        thread::sleep(delay);

                        if let Ok(contents) = clipboard.get_contents() {
                            clipboard
                            .set_contents(uwuifier::uwuify_str_sse(contents.as_str()))
                            .expect("Failed to set clipboard contents");
                        }
                        // delay 
                        thread::sleep(delay);
                        winput::send_inputs(&paste_text);

                        num = 0;
                        punc_count = 0;
                    } else {
                        num += 1;
                    }
                } else if vk == Vk::PrintScreen {
                    process::exit(0);
                }

                previous_key = vk;
            },
            _ => (),
        }
    }
}
