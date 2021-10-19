# cons-shannon 
A Rust program that calculates conservation scores a site in Multiple Sequence Alignment (MSA) by normalized Shannon's entropy. 

## Description 

* It calculates conservation scores by Shannon's entropy [1]. 
* It takes account of stereochemical properties of amino acids, sequence weighting and gap penalty. 

## Dependencies 

* `colored` ( https://github.com/mackwic/colored ) 

``` 
[dependencies]
colored = "2.0"
``` 

## Installation 

You can compile this program by `Cargo`. 📦🦀

[e. g.] 

``` 
% cd cons-shannon-main
% cargo build --release
``` 
Then the object file will be generated in `./target/release` directory.

## Scoring method 

### Conservation score
Conservation scores are calculated by using Shannon's entropy as follows: 

<img width="1440" alt="readme_equation_01" src="https://user-images.githubusercontent.com/83740080/131451510-3e3512df-4db0-4b05-bda2-dd7ab74d4d13.png">

where *N* is the number of amino acid types ( 21 or 7 ) and *K* is the length of a site. That equation normalizes Shannon's entropy so that conserved ( low entropy ) sites score 1 and diverse ( high entropy ) sites score 0 [2]. 

* *AA* = 21 types : The case that residues are not taken account of their stereochemical properties, that is, 20 standard amino acid types and gap. 
* *AA* = 7 types : The case that residues are taken account of their stereochemical properties by classifing them into 7 types as follows [3]:

	* Aliphatic = `{A, V, L, I, M, C}`
	* Aromatic = `{F, W, Y, H}`
	* Polar = `{S, T, N, Q}`
	* Positive = `{K, R}` 
	* Negative = `{D, E}` 
	* Special conformations = `{G, P}` 
	* Gap = `{-}`


### Sequence weighting 
This program supports 2 methods of reducing sequence redundancy, Position-Based Method by Henikoff-Henikoff [4] and Distance-Based Method by Vingron-Argos [5]. 
 
### Gap penalty 

Gap penalties are given to each conservation score by a simple method as follows: 

<img width="1440" alt="readme_equation_02" src="https://user-images.githubusercontent.com/83740080/131451851-78f2a63e-aa0a-4726-8a55-fc9312f9f99b.png">

where *L* is the length of a site and *N* is the number of the gaps in the site. 

## Input file format 

Aligned Multi-FASTA format of amino acid sequences. ⚠️ Note that nucleotide sequences are not supported. 

See some example input files in `demo` directory. 

## Usage 

Major arguments: 

`-i` : Input filename in aligned Multi-FASTA format, REQUIRED. 

`-o` : Output filename, REQUIRED.

`-m` :   Method of sequence weighting ( `hen` or `va`, default `hen` ). 

[e. g.]

```
% ./cons-shannon -i input.fasta -o output.txt -m va -c yes
``` 

Type `-h` to see other available options. 

## Output 

Number`\t`Conservation score`\t`Composition of the site 

[e. g] 

<img width="969" alt="result_picture" src="https://user-images.githubusercontent.com/83740080/131452213-57118f5c-225d-43e0-b4a4-dfb35e6c8378.png">

## References

1. Shannon, Claude E. "A mathematical theory of communication." The Bell system technical journal 27.3 (1948): 379-423. 
2. Valdar, William SJ. "Scoring residue conservation." Proteins: structure, function, and bioinformatics 48.2 (2002): 227-241.
3. Mirny, Leonid A., and Eugene I. Shakhnovich. "Universally conserved positions in protein folds: reading evolutionary signals about stability, folding kinetics and function." Journal of molecular biology 291.1 (1999): 177-196. 
4. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578. 
5. Vingron, Martin, and Patrick Argos. "A fast and sensitive multiple sequence alignment algorithm." Bioinformatics 5.2 (1989): 115-121.
