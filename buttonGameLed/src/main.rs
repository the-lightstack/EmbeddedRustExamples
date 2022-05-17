#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{OutputPin,InputPin};
use embedded_time::rate::*;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;


fn get_random_u16(rosc:&hal::rosc::RingOscillator<hal::rosc::Enabled>)->u16{
    let mut number:u16 = 0;
    for n in 0..16{
        number |= (rosc.get_random_bit() as u16) << n;
    };
    number 
}

#[entry]
fn main() -> !{
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    let sio = hal::Sio::new(pac.SIO);
    
    let rosc = hal::rosc::RingOscillator::new(pac.ROSC);
    let rosc = rosc.initialize();


    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
        );

    let app_timer = hal::timer::Timer::new(pac.TIMER,&mut pac.RESETS);


    // None boilerplate code starts here
    let mut led_pin = pins.led.into_push_pull_output();
    let _button_pin = pins.gpio15.into_pull_up_input();



    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    led_pin.set_low().unwrap();
    delay.delay_ms(500);

    // maximum wait time is 5 seconds or 5000 ms
    const MAX_NUMBER: u16 = 5000;
    
    loop{
        let num = get_random_u16(&rosc);
        let num = num % MAX_NUMBER;
        
        delay.delay_ms(num as u32);
        led_pin.set_high().unwrap();
        delay.delay_ms(100);
        led_pin.set_low().unwrap();
        
        // Start Counter
        let start_counter_value = app_timer.get_counter();

        loop{
            // check if button has been pressed
            if _button_pin.is_low().unwrap(){
                break;
            }
        }
        let end_counter_value = app_timer.get_counter();
        // Time it took for reaction in milliseconds (timer operates in 
        // microseconds, so divide by 1000)
        let diff_ms = (end_counter_value - start_counter_value) / 1000;

        if diff_ms < 240{
            for _ in 1..=5{
                led_pin.set_high().unwrap();
                delay.delay_ms(100);
                led_pin.set_low().unwrap();
                delay.delay_ms(100);
            }
        }else{
            led_pin.set_high().unwrap();
            delay.delay_ms(1000);
            led_pin.set_low().unwrap();
            delay.delay_ms(1000);
        }

        // sleep until next round
        delay.delay_ms(1000);
    }
}
