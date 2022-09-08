
use std::ffi::CStr;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pxtone_sys::{pxtnDescriptor, pxtnError_get_string, pxtnService, pxtnVOMITPREPARATION};

fn main() {
    unsafe {
        // set up audio device
        let host = cpal::default_host();
        let device = host.default_output_device().expect("failed to find output device");
        println!("Output device: {}", device.name().unwrap_or("unknown".to_string()));

        let config = device.default_output_config().unwrap();

        // init pxtone
        let mut serv = pxtnService::new();
        let mut descriptor = pxtnDescriptor::new();

        match serv.init() {
            0 => {}
            n => panic!("{}", CStr::from_ptr(pxtnError_get_string(n)).to_str().unwrap()),
        }
        if !serv.set_destination_quality(config.channels() as i32, config.sample_rate().0 as i32) {
            panic!("serv.set_destination_quality() failed");
        }

        // load ptcop
        let bytes = include_bytes!("sample.ptcop");

        println!("Loading {} bytes", bytes.len());
        descriptor.set_memory_r(bytes as *const _ as *mut _, bytes.len() as i32);

        match serv.read(&mut descriptor) {
            0 => {}
            n => panic!("{}", CStr::from_ptr(pxtnError_get_string(n)).to_str().unwrap()),
        }
        match serv.tones_ready() {
            0 => {}
            n => panic!("{}", CStr::from_ptr(pxtnError_get_string(n)).to_str().unwrap()),
        }

        // print some info
        println!("serv.moo_get_end_clock() = {}", serv.moo_get_end_clock());
        println!("serv.Unit_Num() = {}", serv.Unit_Num());

        // prepare to moo
        let prep = pxtnVOMITPREPARATION {
            start_pos_meas: 0,
            start_pos_sample: 0,
            start_pos_float: 0.0,
            meas_end: 0,
            meas_repeat: 0,
            fadein_sec: 0.0,
            flags: 0,
            master_volume: 0.5,
        };

        if !serv.moo_preparation(&prep) {
            panic!("serv.moo_preparation() failed");
        }
        println!("serv.moo_get_total_sample() = {}", serv.moo_get_total_sample());
        let mut sn = serv.moo_get_total_sample();
        sn = sn - (sn % 4);
        println!("sn = {}", sn);
        let mut mem: Vec<i16> = vec![0; sn as usize * 2];

        // moo
        if !serv.Moo(mem.as_mut_ptr() as *mut _ as *mut _, mem.len() as i32 * 2) {
            panic!("serv.Moo() failed");
        }

        println!("Mooed to buffer, playing back...");

        // play back audio
        let mut sample_i = 0;
        let mut next_value = move || {
            if sample_i >= mem.len() {
                return 0.0;
            }

            let v = mem[sample_i];
            sample_i += 1;
            v as f32 / i16::MAX as f32
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let sample_rate = config.sample_rate().0;
        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, 2, &mut next_value)
            },
            err_fn,
        ).expect("Failed to start audio stream");
        stream.play().expect("Failed to play audio");

        println!("Sleeping for {:.2}s", sn as f64 / sample_rate as f64);
        std::thread::sleep(std::time::Duration::from_secs_f64(sn as f64 / sample_rate as f64));
        println!("Done!");

    }
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where T: cpal::Sample,
{
    let n = output.chunks_mut(channels);
    for frame in n {
        for sample in frame.iter_mut() {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            *sample = value;
        }
    }
}