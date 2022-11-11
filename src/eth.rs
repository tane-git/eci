use crate::reqwest;

// what does making a mod here do?
// pub mod eth {

    // Keeping naming convention of wrapped
    #[allow(non_snake_case)]
    pub fn blockNumber() {
        println!("fetching blockNumber...");

        // let url = String::from("https://github.com/");

        // reqwest::reqwest(&url).unwrap();
        // reqwest::reqwest().unwrap();

        match reqwest::reqwest() {
            Ok(_) => println!("ok"),
            Err(err) => println!("{err}")
        }

        // let res = reqwest::reqwest(&url);
        // println!("{:?}", res);

        // res.unwrap_or_else(|err| println!("{err}"));

        // print!("{res}");
    }
// }