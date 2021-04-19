use cgmapper::Color;
use clap::{App, Arg};
use std::str::FromStr;
use std::{fs::File, process::exit};

fn main() {
    let matches = App::new("cgmapper")
        .version("1.0")
        .author("Maximilian H. <maximilian.huber@mnd.thm.de>")
        .about("Map vertex from graph to color")
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .value_name("FOLDER")
                .help("Path to folder containing necessary information for mapping")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .value_name("FILE")
                .help("Name of file containing graph")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("node")
                .short("n")
                .long("node")
                .value_name("UNSIGNED INTEGER")
                .help("Vertex to be mapped to color")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .possible_values(&["rgb", "hex"])
                .value_name("FORMAT")
                .help("Specify output"),
        )
        .get_matches();

    if let Ok(vertex) = u64::from_str(matches.value_of("node").unwrap()) {
        let path = format!(
            "{}/.{}.json",
            matches.value_of("data").unwrap(),
            matches.value_of("graph").unwrap()
        );
        if let Ok(mut file) = File::open(path) {
            // let color =
            let color = Color::new(&vertex, &mut file);
            let output = matches.value_of("output").unwrap_or("hex");
            match output {
                "hex" => println!("{}", color.get_hex()),
                "rgb" => println!("{:?}", color.get_rgb()),
                _ => unreachable!(),
            }
        } else {
            //invalid path
            exit(-2);
        }
    } else {
        //invalid vertex
        exit(-1)
    }
}
