use ::*;

pub fn input_loop(stop : Arc<Mutex<bool>>, input_tx : mpsc::Sender<u8> ) {

    let mut player_input = async_stdin();
    let mut buf : [u8; 1] = [0];
    loop {
        {
            if *stop.lock().unwrap() {
                break;
            }
        }
        while let Ok(amount) = player_input.read(&mut buf) {
            if amount == 1 {
                match buf[0] {
                    27|b'q' => {
                        *stop.lock().unwrap() = true;
                    },
                    _ => {
                        if let Err(_) = input_tx.send(buf[0]) {};
                    },
                }
            } else {
                break;
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
}
