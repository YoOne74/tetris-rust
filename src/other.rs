fn mysleep(_ms: i32) -> u128 {
    let now = SystemTime::now();
    
    sleep(Duration::new(2, 0));

    match now.elapsed() {
        Ok(elapsed) => {
            // println!("{}",elapsed.as_secs());
            return elapsed.as_millis();
        }
        Err(e) => {

            println!("what the hell {e:?}");
            return 1
        }
    }
}

