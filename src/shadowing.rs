fn main() { 

	// original value of gfg 
	// variable is 100 
	let gfg = 100; 
	
	// gfg variable has value 
	// 100-50 = 50 here 
	// gfg variable got shadowed 
	let gfg = gfg -50; 
	
	// Again gfg variable has value 
	// 50*5 = 250 
	// gfg variable got shadowed again 
	let gfg = gfg * 5; 

	println!("The value of gfg is: {}", gfg); 
}
