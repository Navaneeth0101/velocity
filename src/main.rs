use std::time::{Instant, Duration};
use reqwest::{Error, Response, blocking::Client};
use std::thread;


struct SpeedMeasurements{
    bytes: usize,
    elapsed: Duration,
    mbps: f32
}

impl SpeedMeasurements{
    fn new(bytes: usize, elapsed: Duration, mbps: f32) -> Self {
        SpeedMeasurements{
            bytes,
            elapsed,
            mbps
        }
    }
}


fn main() {


    let client = Client::builder()
    .user_agent("Mozilla/5.0 (X11; Linux x86_64) NetworkSpeedTest/1.0")
    .build()
    .unwrap();
    println!("NETTOOLS V1");

    //http latency

    println!("==================");
    println!("   HTTP LATENCY");
    println!("==================");
    match http_ping(&client, 15) {
        Ok((maxping, minping, avgping, jitter, pingfail)) => {
            println!("max: {maxping}");
            println!("min: {minping}");
            println!("avg: {avgping}");
            println!("jitter: {jitter}");
            println!("failure count: {pingfail}");
        }

        Err(err) => {
            println!("error: {err}");
        }
    }
    
    println!("==================");

    //icmp

     println!("==================");
    println!("   ICMP LATENCY");
    println!("==================");
    match icmp_ping(15) {
        Ok((maxping, minping, avgping, jitter, pingfail)) => {
            println!("max: {maxping}");
            println!("min: {minping}");
            println!("avg: {avgping}");
            println!("jitter: {jitter}");
            println!("failure count: {pingfail}");
        }

        Err(err) => {
            println!("error: {err}");
        }
    }
    
    println!("==================");


    //download speed

    println!("==================");
    println!("  DOWNLOAD SPEED");
    println!("==================");

    match speed_test(&client, 1, download_test_variations){
        Ok((a, b, c, d)) => {
            println!("Max Speed: {a}mbps");
            println!("Min Speed: {b}mbps");
            println!("Avg Speed: {c}mbps");
            println!("Fail count: {d}");

        }
        Err(x) =>{
            println!("Error: {x}");
        }
    }
    //upload speed

    println!("==================");
    println!("   UPLOAD SPEED");
    println!("==================");

    match speed_test(&client, 1, upload_test_variations){
        Ok((a, b, c, d)) => {
            println!("Max Speed: {a}mbps");
            println!("Min Speed: {b}mbps");
            println!("Avg Speed: {c}mbps");
            println!("Fail count: {d}");

        }
        Err(x) =>{
            println!("Error: {x}");
        }
    }
    
}


fn http_ping(client: &Client, num_of_check : usize) -> Result<(f32, f32, f32, f32, i8), i8>{

    let mut totalpings:Vec<f32>  = Vec::new();
    let mut failedpingcount: i8 = 0;
    for _i in 0..num_of_check{
        let start = Instant::now();

        match client.head("https://speed.cloudflare.com/").send(){
            Ok(response) => 
                match response.error_for_status() {
                    Ok(_) => {
                        let elapsed = (start.elapsed().as_secs_f32() * 1000.0 *100.0).round() / 100.0;
                        // println!("ping {}: {:.2} ms", _i + 1, elapsed);
                        totalpings.push(elapsed);
                    },
                    Err(_) => failedpingcount +=1,
                },

            Err(_) => failedpingcount+=1,
        };
    }
    let maxping;
    let minping: f32;
    match totalpings.iter().copied().reduce(f32::max) {
        Some(x) => {
            maxping = x;
        },
        None => {
            return Result::Err(-1)
        }
    }

        match totalpings.iter().copied().reduce(f32::min) {
        Some(x) => {
            minping = x;
        },
        None => {
            return Result::Err(-1)
        }
    }

    let sum: f32 = totalpings.iter().sum();
    let avgping = sum / totalpings.len() as f32;


    //not proud of the readability of the code below 
    //basically shit returns standard deviation of total pings to jitter
    let jitter = (totalpings
                            .iter()
                            .map(|x| (x - avgping).powi(2))
                            .sum::<f32>()
                            / totalpings.len() as f32)
                            .sqrt();
    
    

    return Result::Ok((maxping, minping, avgping, jitter, failedpingcount)); //for now to tell vscode intelisense to fuck off and lemme code in peace
}


fn icmp_ping(num_of_check : usize) ->  Result<(f32, f32, f32, f32, i8), i8> {
    let mut totalpings:Vec<f32>  = Vec::new();
    let mut failedpingcount: i8 = 0;
    let target = "1.1.1.1".parse().unwrap();
    for _i in 0..num_of_check{
        match ping::new(target).timeout(Duration::from_secs(2)).send(){
            Ok(x) => {
                let rtt = x.rtt.as_secs_f32() * 1000.0;
                totalpings.push(rtt);
            },
            Err(_) => failedpingcount += 1        
        }
    }
    let maxping;
    let minping: f32;
    match totalpings.iter().copied().reduce(f32::max) {
        Some(x) => {
            maxping = x;
        },
        None => {
            return Result::Err(-1)
        }
    }

    match totalpings.iter().copied().reduce(f32::min) {
        Some(x) => {
            minping = x;
        },
        None => {
            return Result::Err(-1)
        }
    }

    let sum: f32 = totalpings.iter().sum();
    let avgping = sum / totalpings.len() as f32;


    //not proud of the readability of the code below 
    //basically shit returns standard deviation of total pings to jitter
    let jitter = (totalpings
                            .iter()
                            .map(|x| (x - avgping).powi(2))
                            .sum::<f32>()
                            / totalpings.len() as f32)
                            .sqrt();
    
    

    return Result::Ok((maxping, minping, avgping, jitter, failedpingcount)); //for now to tell vscode intelisense to fuck off and lemme code in peace
}


fn speed_test<F>(client: &Client, multiplier: usize, test_fn: F) -> Result<(f32, f32, f32, i32),i8>
where F: Fn(&Client, usize) -> Result<SpeedMeasurements, Error>

{
    let tests = [
        (100_000, 5), // 1kb x 5
        (1_000_000, 4),  // 1 MB × 4
        (10_000_000, 3), // 10 MB × 3
        (25_000_000, 2),  // 25 MB × 2
        (50_000_000,2) //50MB x 2
    ];

    let mut measurements = Vec::new();
    let mut failcount = 0;

    for (bytes, count) in tests{
        for _ in 0..(count * multiplier) {
            match test_fn(client, bytes){
                Ok(x) => {
                    measurements.push(x);
                }
                Err(e) => {
                    println!("Download failed: {e}");
                    failcount += 1;
                }
            }
            thread::sleep(Duration::from_millis(500));

        }
    }

   // let avgsmall = smallvec.iter().sum::<f32>() / smallvec.len() as f32;
   // let avgmed = medvec.iter().sum::<f32>() / medvec.len() as f32;
   // let avgmed = largevec.iter().sum::<f32>() / largevec.len() as f32;

    // let avg = measurements.iter().map(|x| x.mbps).sum::<f32>() / measurements.len() as f32;
    let min;
    match measurements.iter().map(|x| x.mbps).reduce(f32::min){
        Some(x) => min = x,
        None => min = -1.0,
    }
    let max;
    match measurements.iter().map(|x| x.mbps).reduce(f32::max){
        Some(x) => max = x,
        None => max = -1.0,
    }

    let total_bytes: usize =
    measurements.iter().map(|x| x.bytes).sum();

    let total_time: Duration =
        measurements.iter().map(|x| x.elapsed).sum();

    let avg =
        (total_bytes as f32 * 8.0)
        / total_time.as_secs_f32()
        / 1_000_000.0;

    return Ok((max, min, avg, failcount));
}


fn download_test_variations(client: &Client, bytes: usize) -> Result<SpeedMeasurements, Error>{
    let start = Instant::now();
    let response = client.get(format!("https://speed.cloudflare.com/__down?bytes={bytes}")).send()?.error_for_status()?;

    let data = response.bytes()?;

    let elapsed = start.elapsed();

    let bits = data.len() as f32 * 8.0;
    let seconds = elapsed.as_secs_f32();
    let bps = bits/seconds;
    let mbps = bps/1_000_000.0;
    return Ok(SpeedMeasurements::new(data.len(), elapsed, mbps));
}

fn upload_test_variations(client: &Client, bytes: usize) -> Result<SpeedMeasurements, Error>{
    
    let payload = vec![0u8; bytes];
    let start = Instant::now();
    client.post("https://speed.cloudflare.com/__up").body(payload).send()?.error_for_status()?;

    let elapsed = start.elapsed();
    let bits = bytes as f32 * 8.0;
    let seconds = elapsed.as_secs_f32();
    let bps = bits/seconds;
    let mbps = bps/1_000_000.0;
    return Ok(SpeedMeasurements::new(bytes, elapsed, mbps));
}