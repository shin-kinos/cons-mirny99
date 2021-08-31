
use std::fs::read_to_string;
use std::process;

pub struct Fasta {
	pub sequence : Vec<String>,
	pub title    : Vec<String>,
}

impl Fasta {
	pub fn new( arg : &String ) -> Fasta
	{
		let fin = read_to_string( ( *arg ).as_str() ).expect( "FAILED to open input file" );

		let mut seq_list   : Vec<String> = Vec::new();
		let mut title_list : Vec<String> = Vec::new();
		let mut segment    : Vec<String> = Vec::new();

		/*
		 * Distinguish "title lines" and "sequence lines" in Multi-FASTA file.
		 * title_list = List of title lines
		 * seq_list   = List of sequence lines
		 * segment    = Temporary String to conbine a sequence line separated by "\n"
		*/
		for line in fin.lines() {
			if line.starts_with( ">" ) && segment.is_empty() {
				title_list.push( line.to_string() );
			} else if line.starts_with( ">" ) && !segment.is_empty() {
				title_list.push( line.to_string() );
				seq_list.push( segment.concat() );
				segment.clear();
			} else {
				segment.push( line.to_string() );
			}
		}
		seq_list.push( segment.concat() );
		//segment.clear();
		//segment.shrink_to_fit();

		Fasta {
			sequence : seq_list,
			title    : title_list,
		}
	}

	pub fn check_fasta_info( &self )
	{
		let num_seq   : usize = ( self.sequence ).len();
		let num_title : usize = ( self.title ).len();

		if num_seq != num_title { error_bomb( "seq_title_not_same" ) }

		for i in 1 .. num_seq {
			if ( self.sequence[ 0 ] ).len() != ( self.sequence[ i ] ).len() {
				error_bomb( "seq_len_not_same" );
			}
		}

		println!( "\nThe input file was read correctly.\n" )
	}

	pub fn get_site_list( &self ) -> Vec<String>
	{
		let num_seq  : usize = ( self.sequence ).len();
		let num_site : usize = ( self.sequence[ 0 ] ).to_string().len();

		println!( "Number of the sequences : {}", num_seq );
		println!( "Number of the sites     : {}", num_site );

		let mut site_list : Vec<String> = Vec::new();
		let mut site      : Vec<String> = Vec::new();

		for i in 0 .. num_site {
			for j in 0 .. num_seq {
				let segment : Vec<char> = ( self.sequence[ j ] ).chars().collect();
				site.push( segment[ i ].to_string() );
			}
			site_list.push( site.concat() );
			site.clear();
		}
		//site.shrink_to_fit();

		site_list
	}

}

fn error_bomb( arg : &str )
{
	println!( "ERROR !!!" );

	match arg {
		"seq_title_not_same" => println!( "Inadequate format in Multi-FASTA file." ),
		"seq_len_not_same"   => println!( "The length of each sequences must be same." ),
		_                    => (),
	}

	println!( "\nProgram halted !!!\n" );

	process::exit( 1 );
}
