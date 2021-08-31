
use std::env;
use std::process;

pub struct Options {
	pub input    : String,
	pub output   : String,
	pub method   : String,
	pub tolerate : String,
	pub stereo   : String,
	pub colorize : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_m : &String = &String::from( "hen" );
		let mut arg_t : &String = &String::from( "yes" );
		let mut arg_s : &String = &String::from( "7" );
		let mut arg_c : &String = &String::from( "no" );

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				"-m" => { i += 1; arg_m = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-s" => { i += 1; arg_s = &argv[ i ]; }
				"-c" => { i += 1; arg_c = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] ); }
				_    => { show_usage( &argv[ 0 ] ); }
			}
			i += 1;
		}

		match ( *arg_m ).as_str() {
			"hen" | "va" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_t ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_s ).as_str() {
			"7" | "07" | "21" => (),
			_                 => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_c ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		Options {
			input    : arg_i.to_string(),
			output   : arg_o.to_string(),
			method   : arg_m.to_string(),
			tolerate : arg_t.to_string(),
			stereo   : arg_s.to_string(),
			colorize : arg_c.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set :" );
		println!( "===========================================" );
		println!( "Input filename   : {}", self.input );
		println!( "Onput filename   : {}", self.output );
		println!( "Weighting method : {}", self.method );
		println!( "Tolerate BZXU    : {}", self.tolerate );
		println!( "Types of AA      : {}", self.stereo );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions\n\n", *arg );
	println!( "    -i    Input filename in aligned Multi-FASTA format, REQUIRED." );
	println!( "    -o    Onput filename, REQUIRED." );
	println!( "    -m    Method of sequence weighting ('hen' or 'va', default 'hen').\n              hen : Position-Based method by Henikoff and Henikoff\n              va  : Distance-Based method by Vingron and Argos" );
	println!( "    -t    Tolerate AA types 'B', 'Z', 'X' and 'U' in input file ('yes' or 'no', default 'yes').\n          If 'no', program halts when the input file includes B, Z, X, or U." );
	println!( "    -s    Stereochemical propaties of AA ('7' or '21', default '7').\n          If '7', 21 AAs are classified into 7 types." );
	println!( "    -c    Colorize each AA displayed on the terminal depending on their\n          stereochemical properties ('yes' or 'no', default 'no')."  );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
