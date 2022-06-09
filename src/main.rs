use clap::{Arg, Command};
use exif::{DateTime, Error, Exif, In, Tag, Value};

fn main() {
    println!("Beginning to sort photos...\n");
    let start = std::time::Instant::now();

    let command = Command::new("photosort")
        .version("1.0.0-BETA")
        .about("Sorts photos into directories for year and month.")
        .author("Kevin Barnes");

    let input_dir_arg = Arg::new("input_dir")
        .takes_value(true)
        .help("The directory containing unsorted photos.")
        .required(true);

    let output_dir_arg = Arg::new("output_dir")
        .takes_value(true)
        .help("The directory to put the sorted photos.")
        .required(true);

    let command = command.arg(input_dir_arg).arg(output_dir_arg);

    let matches = command.get_matches();

    let input_dir_str = matches.value_of("input_dir")
        .expect("This can't be None, we said it was required.");
    let output_dir_str = matches.value_of("output_dir")
        .expect("This can't be None, we said it was required.");

    let input_dir = std::fs::read_dir(input_dir_str)
        .expect("Input directory not found.");
    std::fs::read_dir(output_dir_str)
        .expect("Output directory not found.");

    println!("Fetching input files...");
    let input_files = recursively_get_input_files(input_dir);
    println!("Fetched {} input files.\n", input_files.len());

    let mut sorted = 0;
    let mut unknown = 0;

    for input in input_files {
        let exif = get_exif(input.to_str().unwrap());
        if let Err(_err) = exif {
            unknown_image(output_dir_str, input);
            unknown += 1;
            continue;
        }
        let exif = exif.unwrap();
        if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
            match field.value {
                Value::Ascii(ref vec) if !vec.is_empty() => {
                    if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                        std::fs::create_dir_all(
                            format!(
                                "{}/{}/{}",
                                output_dir_str,
                                datetime.year.to_string(),
                                month_name(datetime.month)
                            )
                        ).expect("Failed to create dirs.");
                        let destination = format!(
                            "{}/{}/{}/{}",
                            output_dir_str,
                            datetime.year.to_string(),
                            month_name(datetime.month),
                            input.file_name().unwrap().to_str().unwrap()
                        );
                        println!(
                            "Moved {} from '{}' to '{}'",
                            input.file_name().unwrap().to_str().unwrap(),
                            input.to_str().unwrap(),
                            destination
                        );
                        std::fs::rename(
                            input.to_str().unwrap(),
                            destination
                        ).expect("Failed to move file.");
                        sorted += 1;
                    }
                },
                _ => {},
            }
        } else {
            unknown_image(output_dir_str, input);
            unknown += 1;
        }
    }

    println!(
        "\nSorted {} photos with {} unknowns in {} seconds.",
        sorted,
        unknown,
        start.elapsed().as_secs()
    );
}

fn unknown_image(output_directory: &str, input_file: std::path::PathBuf) {
    std::fs::create_dir_all(
        format!(
            "{}/unknown/",
            output_directory
        )
    ).expect("Failed to create 'unknown' directory.");
    let destination = format!(
        "{}/unknown/{}",
        output_directory,
        input_file.file_name().unwrap().to_str().unwrap()
    );
    println!(
        "Moved {} from '{}' to '{}'",
        input_file.file_name().unwrap().to_str().unwrap(),
        input_file.to_str().unwrap(),
        destination.to_string()
    );
    std::fs::rename(
        input_file.to_str().unwrap(),
        destination
    ).expect("Failed to move file.");
}

fn get_exif(input_file: &str) -> Result<Exif, Error> {
    let exif_reader = exif::Reader::new();
    let file = std::fs::File::open(input_file).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    exif_reader.read_from_container(&mut bufreader)
}

fn month_name(month: u8) -> String {
    return String::from(match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "unknown"
    })
}

fn recursively_get_input_files(input_dir: std::fs::ReadDir) -> Vec<std::path::PathBuf> {
    let mut input_files: Vec<std::path::PathBuf> = vec![];

    for entry in input_dir {
        let path = entry.unwrap().path();
        if path.is_dir() {
            input_files.append(
                &mut recursively_get_input_files(path.read_dir().unwrap())
            );
        } else {
            input_files.push(path);
        }
    }

    return input_files;
}

// let extension = input.extension().unwrap().to_str().unwrap();
// let is_image = match extension {
//     "bmp" => true,
//     "gif" => true,
//     "jpg" => true,
//     "jpeg" => true,
//     "png" => true,
//     "svg" => true,
//     _ => false
// };
// let is_video = match extension {
//     "mp4" => true,
//     "mov" => true,
//     "wmv" => true,
//     "flv" => true,
//     "avi" => true,
//     "mkv" => true,
//     _ => false
// };
