use structopt::StructOpt;

fn main() -> Result<(), &'static str> {
    let args = CLIQuery::from_args();
    let net_bits: u32;

    // Validate entered IP class and initialize net_bits
    match &args.ip_class[..].to_uppercase()[..] {
        "A" => net_bits = 8,
        "B" => net_bits = 16,
        "C" => net_bits = 24,
        _ => return Err("Entered IP class is not correct: (Expected A | B | C | a | b | c )"),
    }

    // Calculate bits for subnet
    let mut subnet_bits: u32 = 0;
    while 2i32.pow(subnet_bits) < args.subnets_count {
        subnet_bits += 1;
    }

    // Check possibility of creating network with values
    let host_bits = 2i32.pow(32 - subnet_bits - net_bits) - 2;
    if host_bits >= args.hosts_count {
        let (bin_mask, dec_mask) = generate_mask(subnet_bits + net_bits);
        println!("Mask: {}", dec_mask);
        if args.show_bin_mask {
            println!("Binary: {}", bin_mask);
        }
        Ok(())
    } else {
        println!(
            "It is not possible to create this network\nnetbits: {}\tsubnetbits: {}\thostbits: {}",
            net_bits,
            subnet_bits,
            32 - (net_bits + subnet_bits),
        );
        std::process::exit(0);
    }
}

#[derive(StructOpt)]
struct CLIQuery {
    ip_class: String,
    subnets_count: i32,
    hosts_count: i32,
    #[structopt(long = "show-mask-binary")]
    show_bin_mask: bool,
}

fn generate_mask(mut bits: u32) -> (String, String) {
    let mut mask: Vec<char> = vec!();

    for iterator in 1..32 {
        // Add 1-bits while bits > 0
        if bits > 0 {
            mask.push('1');
            bits -= 1; 
        } else {
            mask.push('0');
        }

        // Add . every 8 bits to separate octets
        if iterator % 8 == 0 {
            mask.push('.');
        }

    }

    let binary_mask: String = mask.iter().collect();
    let decimal_mask: String = convert_bin_ip_to_dec(&binary_mask);
    (binary_mask, decimal_mask)
}

fn convert_bin_ip_to_dec(binary_ip: &String) -> String {
    let octets: Vec<&str> = binary_ip.split(".").collect();
    let mut decimal_parts: Vec<String> = vec!();

    for binary_part in octets {
        decimal_parts.push(i32::from_str_radix(binary_part, 2).unwrap().to_string());
    }

    let decimal_ip: String = decimal_parts.join(".");

    decimal_ip
}
