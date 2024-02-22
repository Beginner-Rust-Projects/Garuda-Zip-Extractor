use std::env::args; // For Using Arguements Passed in CLI
use std::fs::{self, File}; // For File Handling
use std::io; // For Read Write
use std::path::Path; // For File Path Management
use zip::ZipArchive; // For Reading Zip File
use io::copy; // To Copy from one I/O Stream to other
fn main() {
    // A Cleaner Way to Exit [Fancy]
    std::process::exit(zip_extractor()); //The Output of the function should be i32
}

fn zip_extractor() -> i32 {
    // Contains Our Main Logic
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {:#?} <filepath>", args[0]); //arg[0] is the name of the program
        return 1;
    }
    let file_path = Path::new(&*args[1]);  // &* makes a new reference to the args[1]
    let file = File::open(&file_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap(); //read the Zip File
    println!("{:#?}",archive);
    for i in 0..archive.len() { //Going through Archive Files one by one
        let mut file = archive.by_index(i).unwrap();
        // Defining File Once again but its not shadowing since its in a limited scope (i.e Loop)
        let outpath = match file.enclosed_name() { 
            //Just a Security Feature to check that path is not malicious
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            /* Sometimes there is a comment associated with Archive Files containing
             Extra Metadata information */
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} Comment {}", i, comment);
            }
        }
        if (*file.name()).ends_with("/"){ 
            // Ending with '/' signifies that the file is actually a directory
            println!("File {} extracted to '{}' ",i,outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        }
        else{
            println!("File {} extracted to '{}' ({} bytes)",i,outpath.display(),file.size());
        };
        if let Some(parent) = outpath.parent(){
            if !parent.exists(){
                fs::create_dir_all(&parent).unwrap();
            }
        }
        let mut outfile = File::create(&outpath).unwrap();
        copy(&mut file,&mut outfile).unwrap();
    };

    return 0;
}
