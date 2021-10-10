use structopt::StructOpt;

fn main() -> Result<(), &'static str> {
    let mut args = CLIQuery::from_args();

    // Check entered class of IP address
    if let "A" | "B" | "C" = &args.ip_class[..].to_uppercase()[..] {
        args.ip_class = args.ip_class[..].to_uppercase();
    } else {
        return Err("Entered IP class is not correct")
    }

    Ok(())
}

#[derive(StructOpt)]
struct CLIQuery {
    ip_class: String,
    subnets_count: i32,
    hosts_count: i32,
}
