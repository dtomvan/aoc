match arg.as_str() {
	"1" => day1::main,
	"2" => day2::main,
	"3" => day3::main,
	"4" => day4::main,
    _ => unimplemented!(),
}
