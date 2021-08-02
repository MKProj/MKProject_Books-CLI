mod books;
use books::Book;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name="mkproj_book",
    about="Directly get MKProject Books via Command Line",
)]

enum CLI{
    #[structopt(
        about = "Download a book by specifying the books code & format"
    )]
    Get{
        #[structopt(short, long)]
        code: String, 
        #[structopt(short, long)]
        format: String
    },
    #[structopt(
        about = "Search for a book by specifying a name"
    )]
    Search{
        #[structopt(short, long)]
        name: String
    }
}

#[tokio::main]
async fn main(){
    let books = Book::load("src/json/books.json");
    let cli: CLI = CLI::from_args();

    match cli{
        CLI::Get{
            code, 
            format
        }=> {
            for i in books{
                if code == i.code{
                    println!("Downloading...");
                    i.get(&format).await.expect("Failed to download file...");
                } 
            }
        },
        CLI::Search  {
            name
        }=> {
            let mut result_count = 1;
            println!("Results....");
            for i in books{
                if i.name.starts_with(&name){
                    println!("Result {}: {} [{}]", result_count, i.name, i.code);
                    result_count += 1;
                }
            }
        },
    }
}
